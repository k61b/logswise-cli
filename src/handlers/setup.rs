use crate::setup;
use colored::*;
use figlet_rs::FIGfont;

pub struct SetupHandler {}

impl SetupHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_setup(&self) {
        self.print_banner();
        setup::run_setup();
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
