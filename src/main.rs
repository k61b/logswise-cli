//! # Logswise CLI
//!
//! A command-line tool for note-taking, context-aware suggestions, and AI chat.
//!
//! All configuration is stored in `~/.logswise/setup.json` after running the setup command.

mod chat_handler_v3;
mod chat_session;
mod cli;
mod config;
mod errors;
mod handlers;
mod note_handler;
mod personalization;
mod prompts;
mod router;
mod services;
mod simple_interactive;
mod suggestion_handler_v3;
mod types;
mod utils;
mod validation;

use clap::Parser;
use cli::Cli;
use router::CommandRouter;

fn validate_input(cli: &Cli) {
    if let Some(cli::Commands::Setup {
        import: Some(import_path),
        ..
    }) = &cli.command
    {
        if !std::path::Path::new(import_path).exists() {
            eprintln!("❌ Import file does not exist: {import_path}");
            std::process::exit(1);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    // Input validation
    validate_input(&cli);

    // Create router and handle command
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let mut router = CommandRouter::new();

    // If no command is provided, default to Simple mode
    let command = cli.command.unwrap_or(cli::Commands::Simple);
    rt.block_on(router.route(command));
}
