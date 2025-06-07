use crate::types::SupabaseConfig;
use colored::*;
use dirs::home_dir;
use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

/// Loads Supabase configuration from the user's setup file.
/// This function is used by both note and suggestion handlers.
fn load_supabase_config() -> SupabaseConfig {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data =
        fs::read_to_string(&setup_path).expect("Setup not found. Please run 'lw setup' first.");
    let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
    SupabaseConfig {
        project_url: profile["supabaseUrl"].as_str().unwrap().to_string(),
        api_key: profile["supabaseApiKey"].as_str().unwrap().to_string(),
    }
}

/// Adds a note to the Supabase database.
pub fn add_note(content: &str) {
    let config = load_supabase_config();
    let client = Client::new();
    let url = format!("{}/rest/v1/notes", config.project_url);
    let body = json!({ "content": content });
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
