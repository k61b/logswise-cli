use colored::*;
use dialoguer::{Input, Select};

use crate::chat_handler;
use crate::note_handler;
use crate::suggestion_handler;

/// Runs an interactive session for continuous use
pub fn run_interactive() {
    println!(
        "{}",
        "🚀 Welcome to Logswise Interactive Mode!".cyan().bold()
    );
    println!(
        "{}",
        "Type 'help' for commands, 'exit' to quit".bright_black()
    );
    println!();

    // Show recent notes before the first menu display
    show_startup_notes();

    loop {
        let action_options = vec![
            "💬 Chat",
            "📝 Add Note",
            "📋 View Recent Notes",
            "💡 Get Suggestions",
            "📊 View Stats",
            "❓ Help",
            "🚪 Exit",
        ];

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&action_options)
            .default(0)
            .interact();

        match selection {
            Ok(0) => {
                // Chat
                let message: String = Input::new()
                    .with_prompt("💬 Ask me anything")
                    .interact_text()
                    .unwrap_or_default();

                if !message.trim().is_empty() {
                    chat_handler::chat_with_assistant(&message);
                }
            }
            Ok(1) => {
                // Add Note
                let note: String = Input::new()
                    .with_prompt("📝 What's on your mind?")
                    .interact_text()
                    .unwrap_or_default();

                if !note.trim().is_empty() {
                    note_handler::add_note(&note);
                }
            }
            Ok(2) => {
                // View Recent Notes
                note_handler::show_recent_notes(3);
            }
            Ok(3) => {
                // Get Suggestions
                let query: String = Input::new()
                    .with_prompt("💡 What do you need suggestions for?")
                    .interact_text()
                    .unwrap_or_default();

                if !query.trim().is_empty() {
                    suggestion_handler::get_suggestions(&query);
                }
            }
            Ok(4) => {
                // View Stats
                use crate::handlers::system::SystemHandler;
                let system_handler = SystemHandler::new();
                system_handler.print_stats();
            }
            Ok(5) => {
                // Help
                print_interactive_help();
            }
            Ok(6) => {
                // Exit
                println!(
                    "{}",
                    "👋 Thanks for using Logswise! See you next time.".green()
                );
                break;
            }
            Err(_) => {
                println!("{}", "❌ Invalid selection. Try again.".red());
            }
            _ => {
                println!("{}", "❌ Invalid selection. Try again.".red());
            }
        }
        println!(); // Add spacing between actions
    }
}

fn print_interactive_help() {
    println!("{}", "Interactive Mode Help:".bold().cyan());
    println!(
        "• {} - Have a conversation with your AI assistant",
        "Chat".green()
    );
    println!(
        "• {} - Capture thoughts, ideas, or code snippets",
        "Add Note".green()
    );
    println!("• {} - Show your last 3 notes", "View Recent Notes".green());
    println!(
        "• {} - Get context-aware advice and recommendations",
        "Get Suggestions".green()
    );
    println!(
        "• {} - View your profile and configuration",
        "View Stats".green()
    );
    println!("• {} - Show this help message", "Help".green());
    println!("• {} - Exit interactive mode", "Exit".green());
    println!();
    println!(
        "💡 {}",
        "Tip: All your notes and conversations are automatically saved and used as context!"
            .yellow()
    );
}

/// Shows recent notes with a beautiful UI on startup
fn show_startup_notes() {
    use crate::utils::load_supabase_config;
    use reqwest::blocking::Client;

    // Try to load config and show notes if available
    let config = match load_supabase_config() {
        Ok(cfg) => cfg,
        Err(_) => {
            // If no config, show a friendly message about setup
            println!(
                "{}",
                "┌─ Quick Start ─────────────────────────────────────────────────────────────┐"
                    .bright_black()
            );
            println!(
                "{}  🔧 {}",
                "│".bright_black(),
                "Run 'setup' to configure Logswise for note-taking".cyan()
            );
            println!(
                "{}  📚 {}",
                "│".bright_black(),
                "Then start adding notes and chatting with AI!".cyan()
            );
            println!(
                "{}",
                "└───────────────────────────────────────────────────────────────────────────┘"
                    .bright_black()
            );
            println!();
            return;
        }
    };

    let client = Client::new();
    let url = format!("{}/rest/v1/notes", config.project_url);

    // Fetch recent notes quietly (no spinner for startup)
    let response = client
        .get(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .query(&[
            ("select", "content,created_at"),
            ("order", "created_at.desc"),
            ("limit", "3"),
        ])
        .send();

    match response {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<Vec<serde_json::Value>>() {
                Ok(notes) => {
                    if notes.is_empty() {
                        // No notes yet - show encouraging message
                        println!("{}","┌─ Recent Notes ────────────────────────────────────────────────────────────┐".bright_black());
                        println!(
                            "{}  📝 {}",
                            "│".bright_black(),
                            "No notes yet - add your first note below!".yellow()
                        );
                        println!(
                            "{}  💡 {}",
                            "│".bright_black(),
                            "Notes help provide context for AI suggestions".bright_black()
                        );
                        println!("{}", "└───────────────────────────────────────────────────────────────────────────┘".bright_black());
                    } else {
                        // Show recent notes with beautiful formatting
                        println!("{}", "┌─ Recent Notes ────────────────────────────────────────────────────────────┐".bright_black());

                        for (i, note) in notes.iter().enumerate() {
                            let content = note["content"].as_str().unwrap_or("(empty)");
                            let created_at = note["created_at"].as_str().unwrap_or("unknown time");

                            // Format the timestamp to show just the date
                            let formatted_time = created_at.split('T').next().unwrap_or(created_at);

                            // Truncate long notes for the preview
                            let preview = if content.len() > 54 {
                                format!("{}...", &content[0..54])
                            } else {
                                content.to_string()
                            };

                            let bullet = match i {
                                0 => "●".green(),
                                1 => "●".yellow(),
                                2 => "●".blue(),
                                _ => "●".white(),
                            };

                            println!(
                                "{}  {} {} {}",
                                "│".bright_black(),
                                bullet,
                                preview.white(),
                                format!("({formatted_time})").bright_black()
                            );
                        }

                        println!("{}", "└───────────────────────────────────────────────────────────────────────────┘".bright_black());
                    }
                }
                Err(_) => {
                    // Error parsing - show generic message
                    println!("{}","┌─ Recent Notes ────────────────────────────────────────────────────────────┐".bright_black());
                    println!(
                        "{}  ⚠️  {}",
                        "│".bright_black(),
                        "Unable to load recent notes".yellow()
                    );
                    println!("{}", "└───────────────────────────────────────────────────────────────────────────┘".bright_black());
                }
            }
        }
        Ok(_) | Err(_) => {
            // Network error or HTTP error - show connection message
            println!(
                "{}",
                "┌─ Recent Notes ────────────────────────────────────────────────────────────┐"
                    .bright_black()
            );
            println!(
                "{}  🔌 {}",
                "│".bright_black(),
                "Unable to connect to Supabase".yellow()
            );
            println!(
                "{}  💡 {}",
                "│".bright_black(),
                "Check your internet connection and config".bright_black()
            );
            println!(
                "{}",
                "└───────────────────────────────────────────────────────────────────────────┘"
                    .bright_black()
            );
        }
    }

    // Add a subtle separator before the menu
    println!("{}", "─".repeat(79).bright_black());
    println!();
}
