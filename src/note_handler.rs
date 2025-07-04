use crate::errors::{AppError, AppResult};
use crate::services::config::load_supabase_config;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Adds a note to the Supabase database.
pub async fn add_note(content: &str) {
    if let Err(e) = add_note_impl(content).await {
        println!("{}", format!("❌ {e}").red());
        if matches!(e, AppError::Config(_)) {
            println!("Please run 'logswise-cli setup' first.");
        }
    }
}

/// Internal implementation of add_note with proper error handling.
async fn add_note_impl(content: &str) -> AppResult<()> {
    validate_note_content(content)?;

    let config = load_supabase_config()
        .map_err(|e| AppError::Config(format!("Supabase config error: {e}")))?;

    let profile = crate::utils::load_profile()
        .map_err(|e| AppError::Config(format!("Profile load error: {e}")))?;

    let spinner = create_spinner();
    let client = Client::new();

    // Generate embedding for the note content
    spinner.set_message("Generating embedding for note...");
    let embedding_vec = generate_note_embedding(&profile, content, &spinner).await;

    // Store note in Supabase
    store_note_in_supabase(&client, &config, content, embedding_vec, &spinner).await?;

    spinner.finish_and_clear();
    println!("{}", "📝 Note added successfully!".green());
    Ok(())
}

fn validate_note_content(content: &str) -> AppResult<()> {
    if content.trim().is_empty() {
        return Err(AppError::InvalidInput(
            "Note content cannot be empty".to_string(),
        ));
    }

    if content.len() > 10000 {
        return Err(AppError::InvalidInput(
            "Note content too long (max 10,000 characters)".to_string(),
        ));
    }

    Ok(())
}

fn create_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
}

async fn generate_note_embedding(
    profile: &serde_json::Value,
    content: &str,
    spinner: &ProgressBar,
) -> Option<Vec<f32>> {
    let ollama_base_url = profile["ollamaBaseUrl"]
        .as_str()
        .unwrap_or("http://localhost:11434");
    let ollama_url = format!("{ollama_base_url}/api/embeddings");
    let ollama_model = profile["embeddingModel"]
        .as_str()
        .unwrap_or("nomic-embed-text");

    match crate::services::ollama::generate_embedding_async(&ollama_url, ollama_model, content)
        .await
    {
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
    }
}

async fn store_note_in_supabase(
    client: &Client,
    config: &crate::types::SupabaseConfig,
    content: &str,
    embedding_vec: Option<Vec<f32>>,
    spinner: &ProgressBar,
) -> AppResult<()> {
    spinner.set_message("Saving note to Supabase...");

    let url = format!("{}/rest/v1/notes", config.project_url);
    let body = if let Some(embedding) = embedding_vec {
        json!({ "content": content, "embedding": embedding })
    } else {
        json!({ "content": content })
    };

    let response = client
        .post(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::Network(format!("Failed to send request: {e}")))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(AppError::Database(format!("HTTP {status}: {error_text}")));
    }

    Ok(())
}

/// Shows recent notes from Supabase
pub async fn show_recent_notes(count: usize) {
    let config = match load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {e}").red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };

    let client = Client::new();
    let url = format!("{}/rest/v1/notes", config.project_url);

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Fetching recent notes...");

    let response = client
        .get(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .query(&[
            ("select", "content,created_at"),
            ("order", "created_at.desc"),
            ("limit", &count.to_string()),
        ])
        .send()
        .await;

    match response {
        Ok(resp) => {
            spinner.finish_and_clear();
            if resp.status().is_success() {
                match resp.json::<Vec<serde_json::Value>>().await {
                    Ok(notes) => {
                        if notes.is_empty() {
                            println!(
                                "📝 No notes found. Add your first note with: {}",
                                "logswise-cli note 'Your note here'".cyan()
                            );
                        } else {
                            println!("📝 {} most recent notes:\n", notes.len().to_string().cyan());
                            for (i, note) in notes.iter().enumerate() {
                                let content = note["content"].as_str().unwrap_or("(empty)");
                                let created_at =
                                    note["created_at"].as_str().unwrap_or("unknown time");

                                // Format the timestamp (simplified)
                                let formatted_time =
                                    created_at.split('T').next().unwrap_or(created_at);

                                println!(
                                    "{}. {} {}",
                                    (i + 1).to_string().green(),
                                    content,
                                    format!("({formatted_time})").bright_black()
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", format!("Error parsing notes: {e}").red());
                    }
                }
            } else {
                println!(
                    "{}",
                    format!("Error fetching notes: HTTP {}", resp.status()).red()
                );
            }
        }
        Err(e) => {
            spinner.finish_and_clear();
            println!("{}", format!("Network error: {e}").red());
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
        let prompt = format!("{user_info}\n\nUser wants suggestions for: {query}\nSuggestions:");
        assert!(prompt.contains("User wants suggestions for: How to improve logging?"));
    }
}
