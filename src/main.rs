//! # Logswise CLI
//!
//! A command-line tool for note-taking, context-aware suggestions, and AI chat.
//!
//! All configuration is stored in `~/.logswise/setup.json` after running the setup command.

mod chat_handler;
mod errors;
mod note_handler;
mod performance;
mod setup;
mod suggestion_handler;
mod types;
mod utils;
mod validation;
mod services {
    pub mod ollama;
    pub mod supabase;
}

use clap::{Parser, Subcommand};
use colored::*;
use figlet_rs::FIGfont;

#[derive(Parser)]
#[command(name = "logswise-cli")]
#[command(about = "Logswise CLI - Effortless notes, context-aware suggestions, and AI chat")]
#[command(version)]
#[command(
    long_about = "A command-line tool for note-taking, context-aware suggestions, and AI chat powered by Ollama and Supabase."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run interactive setup for your profile and configuration
    Setup,
    /// Add a note to your collection
    Note {
        /// The content of the note to add
        content: String,
    },
    /// Get context-aware suggestions for a query
    Suggestion {
        /// The query to get suggestions for
        query: String,
    },
    /// Chat with the AI assistant
    Chat {
        /// The message to send to the assistant
        message: String,
    },
    /// Show information about Logswise CLI
    About,
    /// Display your profile and configuration stats
    Stats,
    /// Explain how Logswise works
    How,
    /// Show information about embedding models vs LLMs
    Models,
    /// Show troubleshooting tips for model configuration
    Troubleshoot,
    /// Explain how context is used in suggestions and chat
    Context,
    /// Show detailed help and examples  
    Guide,
    /// Check configuration health and connectivity
    Doctor,
}

fn print_banner() {
    if let Ok(standard_font) = FIGfont::standard() {
        if let Some(figure) = standard_font.convert("Logswise CLI") {
            println!("{}", figure.to_string().cyan());
        }
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

    // Input validation
    match &cli.command {
        Commands::Note { content } => {
            if content.trim().is_empty() {
                eprintln!("❌ Note content cannot be empty");
                std::process::exit(1);
            }
            if content.len() > 10000 {
                eprintln!("❌ Note content too long (max 10,000 characters)");
                std::process::exit(1);
            }
        }
        Commands::Suggestion { query } => {
            if query.trim().is_empty() {
                eprintln!("❌ Query cannot be empty");
                std::process::exit(1);
            }
        }
        Commands::Chat { message } => {
            if message.trim().is_empty() {
                eprintln!("❌ Message cannot be empty");
                std::process::exit(1);
            }
        }
        _ => {}
    }

    match cli.command {
        Commands::Setup => {
            print_banner();
            setup::run_setup();
        }
        Commands::Note { content } => note_handler::add_note(&content),
        Commands::Suggestion { query } => suggestion_handler::get_suggestions(&query),
        Commands::Chat { message } => chat_handler::chat_with_assistant(&message),
        Commands::About => print_about(),
        Commands::Stats => print_stats(),
        Commands::How => print_how(),
        Commands::Models => print_models(),
        Commands::Troubleshoot => print_troubleshoot(),
        Commands::Context => print_context(),
        Commands::Guide => print_help(),
        Commands::Doctor => run_doctor(),
    }
}

fn print_about() {
    println!("\nLogswise CLI: Effortless notes, context-aware suggestions, and AI chat for developers.\n");
    println!("- Take notes, get suggestions, and chat with your local LLM (Ollama)\n- Powered by Rust, Supabase, and your own context\n- Open source, privacy-first, and team-ready\n");
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
                "Ollama Base URL: {}",
                profile_json["ollamaBaseUrl"].as_str().unwrap_or("-")
            );
            println!(
                "Embedding Model: {}",
                profile_json["embeddingModel"].as_str().unwrap_or("-")
            );
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

fn print_help() {
    println!("\n{}\n", "Logswise CLI - Detailed Help".bold().cyan());

    println!("{}", "USAGE:".bold());
    println!("  logswise-cli <COMMAND> [ARGS]\n");

    println!("{}", "COMMANDS:".bold());
    println!(
        "  {}  Run interactive setup for your profile and config",
        "setup".green().bold()
    );
    println!(
        "  {}   Add a note to your collection",
        "note".green().bold()
    );
    println!(
        "  {} Get context-aware suggestions for a query",
        "suggestion".green().bold()
    );
    println!("  {}   Chat with your AI assistant", "chat".green().bold());
    println!(
        "  {}  Display your profile and configuration",
        "stats".green().bold()
    );
    println!(
        "  {}  Show information about Logswise CLI",
        "about".green().bold()
    );
    println!("  {}    Explain how Logswise works", "how".green().bold());
    println!(
        "  {} Show information about embedding models vs LLMs",
        "models".green().bold()
    );
    println!(
        "  {} Show troubleshooting tips",
        "troubleshoot".green().bold()
    );
    println!("  {} Explain how context is used", "context".green().bold());
    println!(
        "  {} Check configuration health and connectivity",
        "doctor".green().bold()
    );
    println!("  {}   Show this detailed help\n", "guide".green().bold());

    println!("{}", "EXAMPLES:".bold());
    println!(
        "  {}  # Set up your profile and configuration",
        "logswise-cli setup".cyan()
    );
    println!(
        "  {}  # Add a quick note",
        "logswise-cli note 'Fixed login bug'".cyan()
    );
    println!(
        "  {}  # Get suggestions",
        "logswise-cli suggestion 'How to improve onboarding?'".cyan()
    );
    println!(
        "  {}  # Chat with AI",
        "logswise-cli chat 'What are the best practices for logging?'".cyan()
    );
    println!(
        "  {}  # Check your configuration\n",
        "logswise-cli stats".cyan()
    );

    println!("{}", "TROUBLESHOOTING:".bold());
    println!(
        "  {}  # Diagnose configuration and connectivity issues",
        "logswise-cli doctor".cyan()
    );
    println!(
        "  {}  # Show model information",
        "logswise-cli models".cyan()
    );
    println!(
        "  {}  # Get troubleshooting tips",
        "logswise-cli troubleshoot".cyan()
    );
    println!(
        "  {}  # Show detailed help and examples",
        "logswise-cli guide".cyan()
    );

    println!("{}", "CONFIGURATION:".bold());
    println!(
        "  Config file: {}/.logswise/setup.json",
        dirs::home_dir().unwrap_or_default().display()
    );
    println!(
        "  Run '{}' to create or update your configuration.\n",
        "logswise-cli setup".cyan()
    );

    println!("{}", "GETTING STARTED:".bold());
    println!("  1. Install Ollama: {}", "https://ollama.ai".blue());
    println!("  2. Pull a model: {}", "ollama pull llama3".cyan());
    println!("  3. Set up Supabase (see SUPABASE_SETUP.md)");
    println!("  4. Run: {}", "logswise-cli setup".cyan());
    println!(
        "  5. Start taking notes: {}",
        "logswise-cli note 'My first note!'".cyan()
    );
}

fn run_doctor() {
    println!("\n{}\n", "🔍 Logswise CLI Health Check".bold().cyan());

    let mut issues_found = 0;

    // Check configuration file exists
    println!("{}", "Checking configuration...".bold());
    let config_result = crate::utils::load_profile();
    let supabase_config_result = crate::utils::load_supabase_config();

    match (&config_result, &supabase_config_result) {
        (Ok(profile), Ok(supabase_config)) => {
            println!("  ✅ Configuration file found and valid");

            // Check required fields
            let required_fields = [
                ("profession", "Profession"),
                ("jobTitle", "Job Title"),
                ("llmName", "LLM Name"),
                ("ollamaBaseUrl", "Ollama Base URL"),
                ("embeddingModel", "Embedding Model"),
            ];

            for (field, display_name) in &required_fields {
                if let Some(value) = profile[field].as_str() {
                    if !value.trim().is_empty() {
                        println!("  ✅ {} configured: {}", display_name, value);
                    } else {
                        println!("  ⚠️  {} is empty", display_name.yellow());
                        issues_found += 1;
                    }
                } else {
                    println!("  ❌ {} missing", display_name.red());
                    issues_found += 1;
                }
            }

            // Check URL formats
            if let Some(ollama_url) = profile["ollamaBaseUrl"].as_str() {
                if crate::validation::validate_url(ollama_url) {
                    println!("  ✅ Ollama URL format valid");
                } else {
                    println!("  ❌ Invalid Ollama URL format: {}", ollama_url.red());
                    issues_found += 1;
                }
            }

            if crate::validation::validate_url(&supabase_config.project_url) {
                println!("  ✅ Supabase URL format valid");
            } else {
                println!("  ❌ Invalid Supabase URL format");
                issues_found += 1;
            }

            if crate::validation::validate_api_key(&supabase_config.api_key) {
                println!("  ✅ Supabase API key format valid");
            } else {
                println!("  ❌ Invalid Supabase API key format");
                issues_found += 1;
            }
        }
        _ => {
            println!("  ❌ Configuration file missing or invalid");
            println!("     Run 'logswise-cli setup' to create configuration");
            issues_found += 1;
        }
    }

    // Test Ollama connectivity (if config exists)
    if let Ok(profile) = config_result {
        println!("\n{}", "Testing Ollama connectivity...".bold());
        let ollama_base_url = profile["ollamaBaseUrl"]
            .as_str()
            .unwrap_or("http://localhost:11434");
        let test_url = format!("{}/api/tags", ollama_base_url);

        match reqwest::blocking::get(&test_url) {
            Ok(response) if response.status().is_success() => {
                println!("  ✅ Ollama server is reachable");

                // Try to test embedding model
                let embedding_model = profile["embeddingModel"]
                    .as_str()
                    .unwrap_or("nomic-embed-text");
                println!("  🔍 Testing embedding model: {}", embedding_model.cyan());

                let client = reqwest::blocking::Client::new();
                let embedding_url = format!("{}/api/embeddings", ollama_base_url);
                match crate::services::ollama::generate_embedding(
                    &client,
                    &embedding_url,
                    embedding_model,
                    "test",
                ) {
                    Ok(_) => println!("  ✅ Embedding model '{}' is working", embedding_model),
                    Err(e) => {
                        println!(
                            "  ⚠️  Embedding model '{}' failed: {}",
                            embedding_model.yellow(),
                            e
                        );
                        issues_found += 1;
                    }
                }

                // Try to test LLM
                let llm_name = profile["llmName"].as_str().unwrap_or("");
                if !llm_name.is_empty() {
                    println!("  🔍 Testing LLM: {}", llm_name.cyan());
                    let generate_url = format!("{}/api/generate", ollama_base_url);
                    match crate::services::ollama::generate_suggestion(
                        &client,
                        &generate_url,
                        llm_name,
                        "test",
                    ) {
                        Ok(_) => println!("  ✅ LLM '{}' is working", llm_name),
                        Err(e) => {
                            println!("  ⚠️  LLM '{}' failed: {}", llm_name.yellow(), e);
                            issues_found += 1;
                        }
                    }
                }
            }
            _ => {
                println!(
                    "  ❌ Cannot reach Ollama server at {}",
                    ollama_base_url.red()
                );
                println!("     Make sure Ollama is running: ollama serve");
                issues_found += 1;
            }
        }
    }

    // Test Supabase connectivity (if config exists)
    if let Ok(supabase_config) = supabase_config_result {
        println!("\n{}", "Testing Supabase connectivity...".bold());
        let test_url = format!("{}/rest/v1/", supabase_config.project_url);

        let client = reqwest::blocking::Client::new();
        match client
            .get(&test_url)
            .header("apikey", &supabase_config.api_key)
            .header(
                "Authorization",
                format!("Bearer {}", &supabase_config.api_key),
            )
            .send()
        {
            Ok(response) if response.status().is_success() => {
                println!("  ✅ Supabase connection successful");
            }
            Ok(response) => {
                println!(
                    "  ⚠️  Supabase responded with status: {}",
                    response.status().to_string().yellow()
                );
                println!("     Check your API key and URL configuration");
                issues_found += 1;
            }
            Err(e) => {
                println!(
                    "  ❌ Failed to connect to Supabase: {}",
                    e.to_string().red()
                );
                issues_found += 1;
            }
        }
    }

    // Summary
    println!("\n{}", "Summary:".bold());
    if issues_found == 0 {
        println!("  🎉 All systems are working correctly!");
        println!("     Your Logswise CLI is ready to use.");
    } else {
        println!(
            "  ⚠️  Found {} issue(s) that need attention",
            issues_found.to_string().yellow()
        );
        println!("     Run 'logswise-cli setup' to fix configuration issues");
        println!("     Check Ollama and Supabase documentation for connectivity issues");
    }

    println!("\n{}", "Useful commands:".bold());
    println!("  logswise-cli setup     # Fix configuration");
    println!("  ollama serve           # Start Ollama server");
    println!("  ollama pull <model>    # Download a model");
    println!("  logswise-cli guide     # Show detailed help");
}
