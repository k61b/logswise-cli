mod setup;
mod note_handler;
mod suggestion_handler;
mod chat_handler;
mod types;

use clap::{Parser, Subcommand};
use colored::*;
use figlet_rs::FIGfont;

#[derive(Parser)]
#[command(name = "lw")] 
#[command(about = "Logswise CLI Application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup,
    Note { content: String },
    Suggestion { query: String },
    Chat { message: String },
}

fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Logswise CLI");
    if let Some(ref fig) = figure {
        println!("{}", fig.to_string().cyan());
    }
    println!("{}", "📝 Take notes, 💡 get suggestions, 🤖 chat with your assistant!".magenta());
    println!("{}", "──────────────────────────────────────────────────────────────".bright_black());
}

fn main() {
    let cli = Cli::parse();
    print_banner();
    match cli.command {
        Commands::Setup => setup::run_setup(),
        Commands::Note { content } => note_handler::add_note(&content),
        Commands::Suggestion { query } => suggestion_handler::get_suggestions(&query),
        Commands::Chat { message } => chat_handler::chat_with_assistant(&message),
    }
}
