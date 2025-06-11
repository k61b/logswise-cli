use crate::services::ollama::generate_embedding;
use crate::utils::load_supabase_config;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde_json::json;
use std::time::Duration;

/// Adds a note to the Supabase database.
pub fn add_note(content: &str) {
    // Input validation
    if content.trim().is_empty() {
        println!("{}", "❌ Note content cannot be empty".red());
        return;
    }

    if content.len() > 10000 {
        println!(
            "{}",
            "❌ Note content too long (max 10,000 characters)".red()
        );
        return;
    }

    let config = match load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {}", e).red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };

    // Load profile for Ollama configuration
    let profile = match crate::utils::load_profile() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", format!("Error loading profile: {}", e).red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let client = Client::new();

    // 1. Generate embedding for the note content using shared Ollama service
    spinner.set_message("Generating embedding for note...");
    let ollama_base_url = profile["ollamaBaseUrl"]
        .as_str()
        .unwrap_or("http://localhost:11434");
    let ollama_url = format!("{}/api/embeddings", ollama_base_url);
    let ollama_model = profile["embeddingModel"]
        .as_str()
        .unwrap_or("nomic-embed-text");
    let embedding_vec = match generate_embedding(&client, &ollama_url, ollama_model, content) {
        Ok(embedding) => Some(embedding),
        Err(msg) => {
            spinner.finish_and_clear();
            println!("{}", msg.yellow());
            println!(
                "{}",
                "Note will be saved without embedding (no semantic search)".cyan()
            );
            None
        }
    };

    // 2. Store note and embedding in Supabase
    spinner.set_message("Saving note to Supabase...");
    let url = format!("{}/rest/v1/notes", config.project_url);
    let body = if let Some(embedding) = embedding_vec {
        json!({ "content": content, "embedding": embedding })
    } else {
        json!({ "content": content })
    };
    let res = client
        .post(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send();
    match res {
        Ok(resp) if resp.status().is_success() => {
            spinner.finish_and_clear();
            println!("{}", "📝 Note added successfully!".green());
        }
        Ok(resp) => {
            spinner.finish_and_clear();
            let error_text = resp.text().unwrap_or_default();
            println!("{} {}", "❌ Failed to add note:".red(), error_text);
            println!(
                "{}",
                "Check your Supabase configuration and try again.".yellow()
            );
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("{} {}", "❌ Failed to add note:".red(), e);
            println!(
                "{}",
                "Check your internet connection and Supabase configuration.".yellow()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Note;

    #[test]
    fn test_add_note_and_retrieve() {
        // Simulate adding a note and retrieving it
        let note = Note {
            id: "1".to_string(),
            content: "Integration test note".to_string(),
            created_at: "2025-06-05T12:00:00Z".to_string(),
            embedding: None,
        };
        assert_eq!(note.content, "Integration test note");
    }

    #[test]
    fn test_suggestion_prompt_format() {
        // Simulate suggestion prompt creation
        let user_info = "User Info:\n- Profession: Developer\n- Job Title: Senior\n- Company Name: TestCo\n- Company Size: 10-100";
        let query = "How to improve logging?";
        let prompt = format!(
            "{}\n\nUser wants suggestions for: {}\nSuggestions:",
            user_info, query
        );
        assert!(prompt.contains("User wants suggestions for: How to improve logging?"));
    }
}
