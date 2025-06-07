mod chat_handler;
mod note_handler;
mod setup;
mod suggestion_handler;
mod types;
mod utils;
mod services {
    pub mod ollama;
    pub mod supabase;
}

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
    About,
    Stats,
}

fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Logswise CLI");
    if let Some(ref fig) = figure {
        println!("{}", fig.to_string().cyan());
    }
    println!(
        "{}",
        "📝 Take notes, 💡 get suggestions, 🤖 chat with your assistant!".magenta()
    );
    println!(
        "{}",
        "──────────────────────────────────────────────────────────────".bright_black()
    );
}

fn main() {
    let cli = Cli::parse();
    print_banner();
    match cli.command {
        Commands::Setup => setup::run_setup(),
        Commands::Note { content } => note_handler::add_note(&content),
        Commands::Suggestion { query } => suggestion_handler::get_suggestions(&query),
        Commands::Chat { message } => chat_handler::chat_with_assistant(&message),
        Commands::About => print_about(),
        Commands::Stats => print_stats(),
    }
}

fn print_about() {
    println!("\nLogswise CLI: Effortless notes, context-aware suggestions, and AI chat for developers.\n");
    println!("- Take notes, get suggestions, and chat with your local LLM (Ollama)\n- Powered by Rust, Supabase, and your own context\n- Open source, privacy-first, and team-ready\n");
    println!("GitHub: https://github.com/k61b/logswise-cli\n");
}

fn print_stats() {
    use dirs::home_dir;
    use std::fs;
    use std::path::PathBuf;
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    // Try to count notes from Supabase if possible (not implemented here, just local info)
    let profile = fs::read_to_string(&setup_path).ok();
    if let Some(profile_str) = profile {
        println!("Profile loaded from ~/.logswise/setup.json");
        if let Ok(profile_json) = serde_json::from_str::<serde_json::Value>(&profile_str) {
            println!(
                "Profession: {}",
                profile_json["profession"].as_str().unwrap_or("-")
            );
            println!(
                "Job Title: {}",
                profile_json["jobTitle"].as_str().unwrap_or("-")
            );
            println!(
                "Company: {} ({} employees)",
                profile_json["companyName"].as_str().unwrap_or("-"),
                profile_json["companySize"].as_str().unwrap_or("-")
            );
            println!("LLM: {}", profile_json["llmName"].as_str().unwrap_or("-"));
            println!(
                "Supabase URL: {}",
                profile_json["supabaseUrl"].as_str().unwrap_or("-")
            );
        }
    } else {
        println!("No profile found. Run 'logswise-cli setup' first.");
    }
    println!(
        "\nNote: For full stats (note count, etc.), future versions will fetch from Supabase.\n"
    );
}
