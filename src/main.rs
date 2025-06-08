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
    Help,
    How,
    Models,
    Troubleshoot,
    Context,
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
        Commands::Help => print_help(),
        Commands::How => print_how(),
        Commands::Models => print_models(),
        Commands::Troubleshoot => print_troubleshoot(),
        Commands::Context => print_context(),
    }
}

fn print_about() {
    println!("\nLogswise CLI: Effortless notes, context-aware suggestions, and AI chat for developers.\n");
    println!("-Take notes, get suggestions, and chat with your local LLM (Ollama)\n- Powered by Rust, Supabase, and your own context\n- Open source, privacy-first, and team-ready\n");
    println!("GitHub: https://github.com/k61b/logswise-cli\n");
}

fn print_stats() {
    match crate::utils::load_profile() {
        Ok(profile_json) => {
            println!("Profile loaded from ~/.logswise/setup.json");
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
        Err(e) => {
            println!("{}", format!("Error loading profile: {}", e).red());
            println!("No profile found. Run 'logswise-cli setup' first.");
        }
    }
    println!(
        "\nNote: For full stats (note count, etc.), future versions will fetch from Supabase.\n"
    );
}

fn print_help() {
    println!("\nLogswise CLI Help\n");
    println!("USAGE:");
    println!("  logswise <COMMAND> [ARGS]\n");
    println!("COMMANDS:");
    println!("  setup           Run interactive setup for your profile and config");
    println!("  note <text>     Add a note");
    println!("  suggestion <q>  Get context-aware suggestions");
    println!("  chat <msg>      Chat with the assistant");
    println!("  stats           Show your profile info");
    println!("  about           About Logswise CLI");
    println!("  how             How Logswise works");
    println!("  models          Model types and modes");
    println!("  troubleshoot    Troubleshooting tips");
    println!("  context         How context is used");
    println!("  help            Show this help message\n");
    println!("EXAMPLES:");
    println!("  logswise note 'Refactored login handler for better error messages'");
    println!("  logswise suggestion 'How to improve onboarding?'");
    println!("  logswise chat 'What is the best way to log errors?'");
}

fn print_how() {
    println!("\nHow Logswise Works:\n");
    println!("- Setup your profile and connect to your own Supabase and LLM (Ollama).");
    println!("- Take notes, get suggestions, and chat—all from the CLI.");
    println!(
        "- Suggestions are generated dynamically using your profile and recent notes as context."
    );
    println!(
        "- No personal data is stored in the cloud—only your notes are synced to your Supabase."
    );
    println!("- Perfect for performance reviews, self-improvement, and transparent team collaboration.\n");
}

fn print_models() {
    println!("\nEmbedding Models vs. LLMs:\n");
    println!("- Embedding Models (e.g., nomic-embed-text, bge-base-en, all-minilm): Used for fast semantic search. Enables embedding-only mode—finds relevant notes, but does not generate new text.");
    println!("- LLMs (e.g., llama3, deepseek-coder, mistral, phi3): Used for generating suggestions and chat responses, always using your profile and relevant notes as context.\n");
    println!("How to choose: Use an LLM for chat/suggestions, or an embedding model for fast search only.");
    println!("Tip: The CLI will tell you which mode is active and how to switch models.\n");
}

fn print_troubleshoot() {
    println!("\nTroubleshooting Model Configuration:\n");
    println!("- If you see a message about embedding-only mode, you are using an embedding model. Switch to an LLM for chat/suggestions.");
    println!("- If chat or suggestion commands do not generate text, check your model name in ~/.logswise/setup.json and ensure your Ollama server is running with the correct model.");
    println!("- For best results: use embedding models for fast search, LLMs for chat/suggestions. You can change your model at any time.\n");
}

fn print_context() {
    println!("\nContext Matters:\n");
    println!("- Both suggestion and chat features always use your profile and the most relevant notes as context for the LLM. This ensures that all responses are tailored to your real work and experience.");
    println!("- In embedding-only mode, only semantic search is performed and relevant notes are shown.\n");
}
