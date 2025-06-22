//! # Logswise CLI
//!
//! A command-line tool for note-taking, context-aware suggestions, and AI chat.
//!
//! All configuration is stored in `~/.logswise/setup.json` after running the setup command.

mod chat_handler;
mod cli;
mod errors;
mod handlers;
mod interactive;
mod note_handler;
mod performance;
mod router;
mod setup;
mod suggestion_handler;
mod types;
mod utils;
mod validation;
mod services {
    pub mod ollama;
    pub mod supabase;
}

use clap::Parser;
use cli::Cli;
use router::CommandRouter;

fn validate_input(cli: &Cli) {
    match &cli.command {
        cli::Commands::Note { content } | cli::Commands::N { content } => {
            if content.trim().is_empty() {
                eprintln!("❌ Note content cannot be empty");
                std::process::exit(1);
            }
            if content.len() > 10000 {
                eprintln!("❌ Note content too long (max 10,000 characters)");
                std::process::exit(1);
            }
        }
        cli::Commands::Suggestion { query } | cli::Commands::S { query } => {
            if query.trim().is_empty() {
                eprintln!("❌ Query cannot be empty");
                std::process::exit(1);
            }
        }
        cli::Commands::Chat { message } | cli::Commands::C { message } => {
            if message.trim().is_empty() {
                eprintln!("❌ Message cannot be empty");
                std::process::exit(1);
            }
        }
        cli::Commands::Completions { shell } => {
            if shell.trim().is_empty() {
                eprintln!("❌ Shell type cannot be empty");
                std::process::exit(1);
            }
        }
        _ => {}
    }
}

fn main() {
    let cli = Cli::parse();

    // Input validation
    validate_input(&cli);

    // Create router and handle command
    let router = CommandRouter::new();
    router.route(cli.command);
}
