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

    loop {
        let action_options = vec![
            "💬 Chat",
            "📝 Add Note",
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
                // Get Suggestions
                let query: String = Input::new()
                    .with_prompt("💡 What do you need suggestions for?")
                    .interact_text()
                    .unwrap_or_default();

                if !query.trim().is_empty() {
                    suggestion_handler::get_suggestions(&query);
                }
            }
            Ok(3) => {
                // View Stats
                use crate::handlers::system::SystemHandler;
                let system_handler = SystemHandler::new();
                system_handler.print_stats();
            }
            Ok(4) => {
                // Help
                print_interactive_help();
            }
            Ok(5) => {
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
