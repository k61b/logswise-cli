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
    pub command: Option<Commands>,
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
    Setup {
        /// Update existing configuration selectively
        #[arg(short, long)]
        update: bool,
        /// Use express setup mode (minimal questions)
        #[arg(short, long)]
        express: bool,
        /// Use template/preset setup mode
        #[arg(short, long)]
        template: bool,
        /// Import configuration from file
        #[arg(short, long)]
        import: Option<String>,
    },
    /// Start the main interactive interface with all features (default mode)
    Simple,
    /// Set up enhanced personalization for better suggestions
    Personalize {
        #[command(subcommand)]
        action: Option<PersonalizeAction>,
    },
    /// Initialize or verify database setup (requires existing Supabase config)
    Init,
}
