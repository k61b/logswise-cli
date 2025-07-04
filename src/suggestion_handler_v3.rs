use colored::*;
use reqwest::Client;

use crate::prompts::registry::PromptManager;
use crate::services::config::{get_ollama_config, load_profile, load_supabase_config};
use crate::services::ollama_streaming::generate_suggestion_streaming;

pub struct StreamingSuggestionHandler {
    #[allow(dead_code)]
    prompt_manager: PromptManager,
    client: Client,
}

impl StreamingSuggestionHandler {
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

    /// Search logs for specific entities (people, places, etc.) mentioned in the query
    async fn search_logs_for_entities(
        &self,
        query: &str,
        config: &crate::types::SupabaseConfig,
    ) -> Result<Vec<String>, String> {
        let entities = self.extract_entities_from_query(query);

        if entities.is_empty() {
            return self.search_all_logs(config).await;
        }

        let mut relevant_notes = Vec::new();
        for entity in entities {
            let entity_notes = self.search_logs_by_keyword(&entity, config).await?;
            relevant_notes.extend(entity_notes);
        }

        relevant_notes.sort();
        relevant_notes.dedup();
        Ok(relevant_notes)
    }

    /// Extract potential entities (names, places, etc.) from the query
    fn extract_entities_from_query(&self, query: &str) -> Vec<String> {
        let mut entities = Vec::new();
        let words: Vec<&str> = query.split_whitespace().collect();

        // Look for capitalized words that might be names
        for word in &words {
            // Remove punctuation and check if it starts with capital letter
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic());
            if !clean_word.is_empty()
                && clean_word.chars().next().unwrap().is_uppercase()
                && clean_word.len() > 2
            {
                // Skip common words that are often capitalized
                if ![
                    "The", "This", "That", "How", "What", "Where", "When", "Why", "Who", "I", "My",
                    "We", "Our", "You", "Your",
                ]
                .contains(&clean_word)
                {
                    entities.push(clean_word.to_string());
                }
            }
        }

        entities
    }

    /// Search logs by keyword using Supabase text search
    async fn search_logs_by_keyword(
        &self,
        keyword: &str,
        config: &crate::types::SupabaseConfig,
    ) -> Result<Vec<String>, String> {
        let url = format!("{}/rest/v1/notes", config.project_url);

        let response = self
            .client
            .get(&url)
            .header("apikey", &config.api_key)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .query(&[
                ("select", "content,created_at"),
                ("content", &format!("ilike.%{keyword}%")),
                ("order", "created_at.desc"),
                ("limit", "20"),
            ])
            .send()
            .await
            .map_err(|e| format!("Failed to search logs: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("Search failed: HTTP {}", response.status()));
        }

        let notes = response
            .json::<Vec<serde_json::Value>>()
            .await
            .map_err(|e| format!("Failed to parse search results: {e}"))?;

        Ok(notes
            .into_iter()
            .filter_map(|note| note["content"].as_str().map(|s| s.to_string()))
            .collect())
    }

    /// Search all logs when no specific entities are found  
    async fn search_all_logs(
        &self,
        config: &crate::types::SupabaseConfig,
    ) -> Result<Vec<String>, String> {
        let url = format!("{}/rest/v1/notes", config.project_url);

        let response = self
            .client
            .get(&url)
            .header("apikey", &config.api_key)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .query(&[
                ("select", "content,created_at"),
                ("order", "created_at.desc"),
                ("limit", "15"), // Get recent logs for context
            ])
            .send()
            .await
            .map_err(|e| format!("Failed to fetch logs: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("Fetch failed: HTTP {}", response.status()));
        }

        let notes = response
            .json::<Vec<serde_json::Value>>()
            .await
            .map_err(|e| format!("Failed to parse logs: {e}"))?;

        let mut results = Vec::new();
        for note in notes {
            if let Some(content) = note["content"].as_str() {
                results.push(content.to_string());
            }
        }

        Ok(results)
    }

    /// Create a simplified context-aware prompt using relevant logs
    async fn create_context_aware_prompt(
        &self,
        query: &str,
        relevant_logs: &[String],
        profile: &serde_json::Value,
    ) -> String {
        let profession = profile
            .get("profession")
            .and_then(|v| v.as_str())
            .unwrap_or("Professional");

        let logs_context = self.format_logs_context(relevant_logs);

        format!(
            "You are a helpful AI assistant providing advice to a {profession}.\n\n\
            {logs_context}\n\n\
            QUERY: {query}\n\n\
            INSTRUCTIONS:\n\
            - Use the log entries above for context about people/situations mentioned\n\
            - Reference specific past experiences when applicable\n\
            - Provide actionable advice based on the documented information\n\n\
            RESPONSE:"
        )
    }

    fn format_logs_context(&self, logs: &[String]) -> String {
        if logs.is_empty() {
            return "No relevant logs found.".to_string();
        }

        let formatted_logs: Vec<String> = logs
            .iter()
            .take(3)
            .enumerate()
            .map(|(i, log)| {
                let truncated = if log.len() > 150 {
                    format!("{}...", &log[..150])
                } else {
                    log.clone()
                };
                format!("{}. {}", i + 1, truncated)
            })
            .collect();

        format!("RELEVANT LOG ENTRIES:\n{}", formatted_logs.join("\n"))
    }

    pub async fn get_suggestions(&mut self, query: &str) -> Result<(), String> {
        let profile = load_profile().map_err(|e| format!("Error loading profile: {e}"))?;

        match load_supabase_config() {
            Ok(config) => {
                self.get_context_aware_suggestions(query, &config, &profile)
                    .await
            }
            Err(_) => self.get_simple_suggestions(query, &profile).await,
        }
    }

    async fn get_context_aware_suggestions(
        &self,
        query: &str,
        config: &crate::types::SupabaseConfig,
        profile: &serde_json::Value,
    ) -> Result<(), String> {
        println!(
            "\n{} {}",
            "🔍".bright_blue(),
            "Searching your logs for relevant context...".bright_blue()
        );

        let relevant_logs = self
            .search_logs_for_entities(query, config)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Warning: Log search failed: {e}");
                Vec::new()
            });

        if !relevant_logs.is_empty() {
            println!(
                "{} {}",
                "📋".bright_green(),
                format!(
                    "Found {} relevant entries in your logs",
                    relevant_logs.len()
                )
                .bright_green()
            );
        }

        let prompt = self
            .create_context_aware_prompt(query, &relevant_logs, profile)
            .await;
        let (ollama_url, ollama_model) = get_ollama_config(profile);

        println!(
            "\n{} {}",
            "💡".bright_green(),
            "Context-Aware AI Suggestions:".bright_green().bold()
        );
        if !relevant_logs.is_empty() {
            println!(
                "{}",
                format!(
                    "   📊 Based on {} relevant log entries",
                    relevant_logs.len()
                )
                .bright_black()
            );
        }

        match generate_suggestion_streaming(&ollama_url, &ollama_model, &prompt).await {
            Ok(_) => {
                let tip = if relevant_logs.is_empty() {
                    "💡 Add more notes about people and situations for better context-aware suggestions!"
                } else {
                    "💡 These suggestions are based on your logged experiences."
                };
                println!("\n{}", tip.bright_blue());
                Ok(())
            }
            Err(e) => {
                println!("{}", format!("Error generating suggestions: {e}").red());
                Err(e)
            }
        }
    }

    async fn get_simple_suggestions(
        &self,
        query: &str,
        profile: &serde_json::Value,
    ) -> Result<(), String> {
        println!(
            "\n{} {}",
            "💡".bright_green(),
            "AI Suggestions:".bright_green().bold()
        );
        println!(
            "{}",
            "   ⚠️  Add Supabase configuration for context-aware suggestions".bright_yellow()
        );

        let prompt = format!(
            "You are a helpful AI assistant providing suggestions to a professional. Please provide helpful advice for: {query}"
        );

        let (ollama_url, ollama_model) = get_ollama_config(profile);

        match generate_suggestion_streaming(&ollama_url, &ollama_model, &prompt).await {
            Ok(_) => {
                println!(
                    "\n{}",
                    "💡 Run 'logswise-cli setup' to enable context-aware suggestions!"
                        .bright_blue()
                );
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for StreamingSuggestionHandler {
    fn default() -> Self {
        Self::new()
    }
}

// Async legacy function for backward compatibility
pub async fn get_suggestions_streaming(query: &str) {
    let mut handler = StreamingSuggestionHandler::new();
    if let Err(e) = handler.get_suggestions(query).await {
        eprintln!("Failed to get suggestions: {e}");
    }
}
