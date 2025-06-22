use colored::*;

pub struct HelpHandler {}

impl HelpHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_about(&self) {
        println!("\nLogswise CLI: Effortless notes, context-aware suggestions, and AI chat for developers.\n");
        println!("- Take notes, get suggestions, and chat with your local LLM (Ollama)\n- Powered by Rust, Supabase, and your own context\n- Open source, privacy-first, and team-ready\n");
        println!("GitHub: https://github.com/k61b/logswise-cli\n");
    }

    pub fn print_how(&self) {
        println!("\nHow Logswise Works:\n");
        println!("- Setup your profile and connect to your own Supabase and LLM (Ollama).");
        println!("- Take notes, get suggestions, and chat—all from the CLI.");
        println!("- Suggestions are generated dynamically using your profile and recent notes as context.");
        println!("- No personal data is stored in the cloud—only your notes are synced to your Supabase.");
        println!("- Perfect for performance reviews, self-improvement, and transparent team collaboration.\n");
    }

    pub fn print_models(&self) {
        println!("\nEmbedding Models vs. LLMs:\n");
        println!("- Embedding Models (e.g., nomic-embed-text, bge-base-en, all-minilm): Used for fast semantic search. Enables embedding-only mode—finds relevant notes, but does not generate new text.");
        println!("- LLMs (e.g., llama3, deepseek-coder, mistral, phi3): Used for generating suggestions and chat responses, always using your profile and relevant notes as context.\n");
        println!("How to choose: Use an LLM for chat/suggestions, or an embedding model for fast search only.");
        println!("Tip: The CLI will tell you which mode is active and how to switch models.\n");
    }

    pub fn print_troubleshoot(&self) {
        println!("\nTroubleshooting Model Configuration:\n");
        println!("- If you see a message about embedding-only mode, you are using an embedding model. Switch to an LLM for chat/suggestions.");
        println!("- If chat or suggestion commands do not generate text, check your model name in ~/.logswise/setup.json and ensure your Ollama server is running with the correct model.");
        println!("- For best results: use embedding models for fast search, LLMs for chat/suggestions. You can change your model at any time.\n");
    }

    pub fn print_context(&self) {
        println!("\nContext Matters:\n");
        println!("- Both suggestion and chat features always use your profile and the most relevant notes as context for the LLM. This ensures that all responses are tailored to your real work and experience.");
        println!("- In embedding-only mode, only semantic search is performed and relevant notes are shown.\n");
    }

    pub fn print_guide(&self) {
        println!("\n{}\n", "Logswise CLI - Detailed Help".bold().cyan());

        println!("{}", "USAGE:".bold());
        println!("  logswise-cli <COMMAND> [ARGS]\n");

        println!("{}", "COMMANDS:".bold());
        println!(
            "  {}  Run interactive setup for your profile and config",
            "setup".green().bold()
        );
        println!(
            "  {} (or {})   Add a note to your collection",
            "note".green().bold(),
            "n".green()
        );
        println!(
            "  {} (or {}) Get context-aware suggestions for a query",
            "suggestion".green().bold(),
            "s".green()
        );
        println!(
            "  {} (or {})   Chat with your AI assistant",
            "chat".green().bold(),
            "c".green()
        );
        println!(
            "  {}  Start interactive mode for continuous use",
            "interactive".green().bold()
        );
        println!(
            "  {}  Show recent notes (default: 5)",
            "recent".green().bold()
        );

        println!(
            "  {}  Generate shell completions",
            "completions".green().bold()
        );
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
            "  {}  # Add a quick note (short form)",
            "logswise-cli n 'Fixed login bug'".cyan()
        );
        println!(
            "  {}  # Get suggestions (short form)",
            "logswise-cli s 'How to improve onboarding?'".cyan()
        );
        println!(
            "  {}  # Quick chat (short form)",
            "logswise-cli c 'Best practices for logging?'".cyan()
        );
        println!(
            "  {}  # Start interactive mode",
            "logswise-cli interactive".cyan()
        );
        println!(
            "  {}  # View recent notes",
            "logswise-cli recent --count 10".cyan()
        );

        println!(
            "  {}  # Generate Bash completions",
            "logswise-cli completions bash".cyan()
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
}
