use crate::config::{updater::ConfigUpdater, ConfigManager};
use colored::*;
use figlet_rs::FIGfont;

pub struct SetupHandler {
    config_manager: ConfigManager,
}

impl SetupHandler {
    pub fn new() -> Self {
        Self {
            config_manager: ConfigManager::load_or_create()
                .unwrap_or_else(|_| ConfigManager::new()),
        }
    }

    pub async fn run_smart_setup(
        &mut self,
        update: bool,
        express: bool,
        template: bool,
        import: Option<String>,
    ) {
        self.print_banner();

        if update {
            if let Err(e) =
                ConfigUpdater::run_selective_update(&mut self.config_manager.templates).await
            {
                eprintln!("{} {}", "Error:".red(), e);
                std::process::exit(1);
            }
        } else if express {
            if let Err(e) = self.config_manager.run_express_setup().await {
                eprintln!("{} {}", "Error:".red(), e);
                std::process::exit(1);
            }
        } else if template {
            if let Err(e) = self.config_manager.setup_from_template().await {
                eprintln!("{} {}", "Error:".red(), e);
                std::process::exit(1);
            }
        } else if let Some(import_path) = import {
            // Handle import logic
            println!(
                "{}",
                format!("Importing configuration from: {import_path}").cyan()
            );
            if let Err(e) = self.config_manager.import_existing_setup() {
                eprintln!("{} {}", "Error:".red(), e);
                std::process::exit(1);
            }
        } else {
            // Default smart setup
            if let Err(e) = self.config_manager.run_smart_setup().await {
                eprintln!("{} {}", "Error:".red(), e);
                std::process::exit(1);
            }
        }

        // Save the updated config manager
        if let Err(e) = self.config_manager.save() {
            eprintln!(
                "{} Failed to save configuration: {}",
                "Warning:".yellow(),
                e
            );
        }
    }

    fn print_banner(&self) {
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
}
