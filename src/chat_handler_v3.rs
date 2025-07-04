use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::time::Duration;

use crate::prompts::registry::PromptManager;
use crate::services::ollama::generate_embedding;
use crate::services::ollama_streaming::generate_chat_streaming;
use crate::services::supabase::semantic_search_notes;
use crate::utils::{load_profile, load_supabase_config};

pub struct StreamingChatHandler {
    #[allow(dead_code)]
    prompt_manager: PromptManager,
    #[allow(dead_code)]
    client: Client,
}

impl StreamingChatHandler {
    pub fn new() -> Self {
        let mut prompt_manager = PromptManager::new();
        if let Err(e) = prompt_manager.load_registry() {
            eprintln!("Warning: Could not load prompt registry: {e}");
        }

        Self {
            prompt_manager,
            client: Client::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_assistant(&mut self, message: &str) -> Result<(), String> {
        let profile = load_profile().map_err(|e| format!("Error loading profile: {e}"))?;

        // Try to get notes context, but don't fail if it doesn't work
        let notes_context = match load_supabase_config() {
            Ok(config) => self
                .get_notes_context_simple(&config, message)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Warning: Could not get notes context: {e}");
                    "No relevant notes found.".to_string()
                }),
            Err(_) => {
                "No notes context available - add Supabase configuration for context-aware chat."
                    .to_string()
            }
        };

        // Generate chat response using the new streaming prompt system
        self.generate_chat_response_streaming(&profile, message, &notes_context)
            .await
    }

    /// Simplified notes context method that just searches by keyword
    #[allow(dead_code)]
    async fn get_notes_context_simple(
        &self,
        config: &crate::types::SupabaseConfig,
        message: &str,
    ) -> Result<String, String> {
        // Simple keyword-based search instead of embeddings
        let url = format!("{}/rest/v1/notes", config.project_url);

        // Extract potential keywords from message (similar to entity extraction)
        let keywords = self.extract_keywords_from_message(message);

        let mut relevant_notes = Vec::new();

        for keyword in keywords.iter().take(2) {
            // Limit to 2 keywords to keep it fast
            let response = self
                .client
                .get(&url)
                .header("apikey", &config.api_key)
                .header("Authorization", format!("Bearer {}", config.api_key))
                .query(&[
                    ("select", "content"),
                    ("content", &format!("ilike.%{keyword}%")),
                    ("order", "created_at.desc"),
                    ("limit", "3"), // Keep it very limited
                ])
                .send()
                .await
                .map_err(|e| format!("Failed to search notes: {e}"))?;

            if response.status().is_success() {
                let notes = response
                    .json::<Vec<serde_json::Value>>()
                    .await
                    .map_err(|e| format!("Failed to parse notes: {e}"))?;

                for note in notes {
                    if let Some(content) = note["content"].as_str() {
                        // Truncate long notes
                        let truncated = if content.len() > 100 {
                            format!("{}...", &content[..100])
                        } else {
                            content.to_string()
                        };
                        relevant_notes.push(truncated);
                    }
                }
            }
        }

        // Remove duplicates and limit total
        relevant_notes.sort();
        relevant_notes.dedup();
        relevant_notes.truncate(3);

        let context = if relevant_notes.is_empty() {
            "No relevant notes found.".to_string()
        } else {
            format!("Relevant notes: {}", relevant_notes.join("; "))
        };

        Ok(context)
    }

    /// Extract keywords from chat message for simple search
    #[allow(dead_code)]
    fn extract_keywords_from_message(&self, message: &str) -> Vec<String> {
        let words: Vec<&str> = message.split_whitespace().collect();
        let mut keywords = Vec::new();

        for word in words {
            let clean_word = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();
            if clean_word.len() > 3
                && ![
                    "this", "that", "with", "from", "they", "have", "will", "what", "when",
                    "where", "how",
                ]
                .contains(&clean_word.as_str())
            {
                keywords.push(clean_word);
            }
        }

        keywords.truncate(3); // Limit keywords
        keywords
    }

    #[allow(dead_code)]
    async fn get_notes_context(
        &self,
        config: &crate::types::SupabaseConfig,
        profile: &serde_json::Value,
        message: &str,
    ) -> Result<String, String> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Searching your notes for context...");
        pb.enable_steady_tick(Duration::from_millis(100));

        // First generate embedding for the message
        let ollama_embedding_url = format!(
            "{}/api/embeddings",
            profile
                .get("ollamaBaseUrl")
                .and_then(|v| v.as_str())
                .unwrap_or("http://localhost:11434")
        );
        let ollama_embedding_model = profile
            .get("embeddingModel")
            .and_then(|v| v.as_str())
            .unwrap_or("nomic-embed-text");

        let query_embedding = match generate_embedding(
            &self.client,
            &ollama_embedding_url,
            ollama_embedding_model,
            message,
        )
        .await
        {
            Ok(embedding) => embedding,
            Err(e) => {
                pb.finish_with_message("⚠️  Could not generate embedding");
                return Err(format!("Embedding error: {e}"));
            }
        };

        // Search for relevant notes
        let notes = semantic_search_notes(&self.client, config, &query_embedding, 5).await;

        let notes_context = if notes.is_empty() {
            "\n\nNo relevant notes found in your database.".to_string()
        } else {
            format!(
                "\n\nRelevant context from your notes:\n{}",
                notes.join("\n")
            )
        };

        pb.finish_with_message("✅ Found relevant context");
        Ok(notes_context)
    }

    #[allow(dead_code)]
    async fn generate_chat_response_streaming(
        &mut self,
        profile: &serde_json::Value,
        message: &str,
        notes_context: &str,
    ) -> Result<(), String> {
        // Create a simple chat prompt
        let profession = profile
            .get("profession")
            .and_then(|v| v.as_str())
            .unwrap_or("Professional");
        let _job_title = profile
            .get("jobTitle")
            .and_then(|v| v.as_str())
            .unwrap_or("Team Member");
        let _language = profile
            .get("preferredLanguage")
            .and_then(|v| v.as_str())
            .unwrap_or("General");

        let rendered_prompt = format!(
            "You are a helpful AI assistant chatting with a {profession}.\n\n\
            Context: {notes_context}\n\n\
            User: {message}\n\n\
            Respond in a friendly, conversational way:"
        );

        // Get Ollama configuration
        let ollama_url = format!(
            "{}/api/generate",
            profile
                .get("ollamaBaseUrl")
                .and_then(|v| v.as_str())
                .unwrap_or("http://localhost:11434")
        );
        let ollama_model = profile
            .get("llmName")
            .and_then(|v| v.as_str())
            .unwrap_or("llama3");

        // Show header before streaming starts
        println!("\n{}", "🤖 Assistant:".bright_green().bold());

        // Generate response using streaming Ollama with enhanced chat feedback
        let result = generate_chat_streaming(&ollama_url, ollama_model, &rendered_prompt).await;

        match result {
            Ok(_) => {
                // Response was already printed by the streaming function
                Ok(())
            }
            Err(e) => {
                println!("{}", format!("Error generating response: {e}").red());
                Err(e)
            }
        }
    }
}

impl Default for StreamingChatHandler {
    fn default() -> Self {
        Self::new()
    }
}

// Async legacy function for backward compatibility
#[allow(dead_code)]
pub async fn chat_with_assistant_streaming(message: &str) {
    let mut handler = StreamingChatHandler::new();
    if let Err(e) = handler.chat_with_assistant(message).await {
        eprintln!("Failed to chat with assistant: {e}");
    }
}
