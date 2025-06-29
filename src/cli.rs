use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "logswise-cli")]
#[command(about = "Logswise CLI - Effortless notes, context-aware suggestions, and AI chat")]
#[command(version)]
#[command(
    long_about = "A command-line tool for note-taking, context-aware suggestions, and AI chat powered by Ollama and Supabase."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum PersonalizeAction {
    /// Run the full personalization setup
    Setup,
    /// Update existing personalization settings
    Update,
    /// Show current personalization settings
    Show,
    /// Provide feedback on suggestions
    Feedback {
        /// Category of the suggestion to provide feedback on
        category: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run interactive setup for your profile and configuration
    Setup,
    /// Add a note to your collection
    Note {
        /// The content of the note to add
        content: String,
    },
    /// Add a note to your collection (alias for 'note')
    N {
        /// The content of the note to add
        content: String,
    },
    /// Get context-aware suggestions for a query
    Suggestion {
        /// The query to get suggestions for
        query: String,
    },
    /// Get context-aware suggestions for a query (alias for 'suggestion')
    S {
        /// The query to get suggestions for
        query: String,
    },
    /// Chat with the AI assistant
    Chat {
        /// The message to send to the assistant
        message: String,
    },
    /// Chat with the AI assistant (alias for 'chat')
    C {
        /// The message to send to the assistant
        message: String,
    },
    /// Start interactive mode for continuous note-taking and chatting
    Interactive,
    /// Set up enhanced personalization for better suggestions
    Personalize {
        #[command(subcommand)]
        action: Option<PersonalizeAction>,
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
    /// Show recent notes
    Recent {
        /// Number of recent notes to show (default: 5)
        #[arg(short, long, default_value = "5")]
        count: usize,
    },
    /// Generate shell completions
    Completions {
        /// Shell type (bash, zsh, fish, powershell)
        shell: String,
    },
    /// Initialize or verify database setup (requires existing Supabase config)
    Init,
}
