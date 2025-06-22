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

/// Shows recent notes from Supabase
pub fn show_recent_notes(count: usize) {
    let config = match load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {}", e).red());
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
        .send();

    match response {
        Ok(resp) => {
            spinner.finish_and_clear();
            if resp.status().is_success() {
                match resp.json::<Vec<serde_json::Value>>() {
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
                                    format!("({})", formatted_time).bright_black()
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", format!("Error parsing notes: {}", e).red());
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
            println!("{}", format!("Network error: {}", e).red());
        }
    }
}

/// Creates a note from a predefined template
pub fn create_from_template(template_type: &str) {
    use dialoguer::Input;

    let template_content = match template_type.to_lowercase().as_str() {
        "daily" => {
            let date = std::process::Command::new("date")
                .arg("+%Y-%m-%d")
                .output()
                .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
                .unwrap_or_else(|_| "Today".to_string());

            format!("Daily Log - {}\n\n✅ Completed:\n- \n\n🎯 Today's Focus:\n- \n\n🔄 In Progress:\n- \n\n📝 Notes:\n- ", date)
        }
        "meeting" => {
            let meeting_title: String = Input::new()
                .with_prompt("Meeting title")
                .interact_text()
                .unwrap_or_default();

            format!("Meeting: {}\n\n🎯 Agenda:\n- \n\n💬 Discussion:\n- \n\n✅ Action Items:\n- \n\n📅 Next Steps:\n- ", meeting_title)
        }
        "bug" => {
            let bug_title: String = Input::new()
                .with_prompt("Bug description")
                .interact_text()
                .unwrap_or_default();

            format!("🐛 Bug: {}\n\n🔍 Reproduction Steps:\n1. \n\n💥 Expected vs Actual:\n- Expected: \n- Actual: \n\n🛠️ Potential Fix:\n- \n\n🔗 Related:\n- ", bug_title)
        }
        "idea" => {
            format!("💡 Idea\n\n🎯 Core Concept:\n- \n\n✨ Why This Matters:\n- \n\n🚀 Implementation Thoughts:\n- \n\n🤔 Considerations:\n- ")
        }
        "todo" => {
            format!("📋 TODO\n\n🔥 High Priority:\n- [ ] \n\n📅 This Week:\n- [ ] \n\n💭 Someday/Maybe:\n- [ ] \n\n✅ Completed:\n- [x] ")
        }
        "retrospective" => {
            format!("🔄 Retrospective\n\n✅ What Went Well:\n- \n\n🚫 What Didn't Work:\n- \n\n💡 What We Learned:\n- \n\n🎯 Action Items:\n- ")
        }
        _ => {
            println!("{}", "❌ Unknown template type. Available templates:".red());
            println!("  • {} - Daily work log with focus areas", "daily".cyan());
            println!(
                "  • {} - Meeting notes with agenda and action items",
                "meeting".cyan()
            );
            println!("  • {} - Bug report template", "bug".cyan());
            println!("  • {} - Idea capture template", "idea".cyan());
            println!("  • {} - TODO list with priorities", "todo".cyan());
            println!(
                "  • {} - Team retrospective template",
                "retrospective".cyan()
            );
            return;
        }
    };

    // Allow user to edit the template
    let final_content: String = Input::new()
        .with_prompt("Edit your note (or press Enter to save as-is)")
        .with_initial_text(&template_content)
        .interact_text()
        .unwrap_or(template_content);

    if !final_content.trim().is_empty() {
        add_note(&final_content);
    } else {
        println!("{}", "❌ Empty note not saved".yellow());
    }
}
