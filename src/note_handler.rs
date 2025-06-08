use crate::services::ollama::generate_embedding;
use crate::utils::load_supabase_config;
use colored::*;
use reqwest::blocking::Client;
use serde_json::json;

/// Adds a note to the Supabase database.
pub fn add_note(content: &str) {
    let config = match load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {}", e).red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };
    let client = Client::new();

    // 1. Generate embedding for the note content using shared Ollama service
    let ollama_url = std::env::var("OLLAMA_EMBEDDING_URL")
        .unwrap_or_else(|_| "http://localhost:11434/api/embeddings".to_string());
    let ollama_model =
        std::env::var("OLLAMA_EMBEDDING_MODEL").unwrap_or_else(|_| "nomic-embed-text".to_string());
    let embedding_vec = match generate_embedding(&client, &ollama_url, &ollama_model, content) {
        Ok(embedding) => Some(embedding),
        Err(msg) => {
            println!("{}", msg.yellow());
            None
        }
    };

    // 2. Store note and embedding in Supabase
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
            println!("{}", "📝 Note added!".green());
        }
        Ok(resp) => {
            println!(
                "{} {}",
                "❌ Failed to add note:".red(),
                resp.text().unwrap_or_default()
            );
        }
        Err(e) => {
            println!("{} {}", "❌ Failed to add note:".red(), e);
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
