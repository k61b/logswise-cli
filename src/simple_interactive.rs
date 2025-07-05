use colored::*;
use dialoguer::{Input, Select};
use std::io;

use crate::chat_session;
use crate::config;
use crate::handlers::{help::HelpHandler, system::SystemHandler};
use crate::note_handler;
use crate::suggestion_handler_v3;

/// Main interactive interface with all features
pub async fn run_simple_interactive() -> Result<(), String> {
    // Show welcome screen
    show_welcome();

    // Check configuration first
    check_configuration().await;

    // Main interactive loop
    loop {
        // Clear screen and show menu
        print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
        show_main_menu();

        // Get user selection with arrow keys and number shortcuts
        let selection = show_menu_selector().map_err(|e| format!("Menu selection error: {e}"))?;

        match selection {
            0 => {
                // Chat with AI
                println!("\n{}", "💬 Starting AI Chat Session...".cyan().bold());
                println!("{}", "Press Ctrl+C to return to menu".bright_black());

                if let Err(e) = chat_session::start_chat_session().await {
                    println!("{}", format!("Chat error: {e}").red());
                    pause_for_user().await;
                }
            }
            1 => {
                // Add new log entry
                println!("\n{}", "📝 Add New Log Entry".cyan().bold());
                let note_content = get_user_input(
                    "What would you like to log?",
                    "Type your thoughts, ideas, or notes here...",
                )
                .await?;

                if !note_content.trim().is_empty() {
                    note_handler::add_note(&note_content).await;
                    println!("{}", "✅ Log entry saved successfully!".green());
                } else {
                    println!("{}", "ℹ️  No content entered, returning to menu.".yellow());
                }
                pause_for_user().await;
            }
            2 => {
                // Get AI suggestions
                println!("\n{}", "💡 Get AI Suggestions".cyan().bold());
                let query = get_user_input(
                    "What do you need suggestions for?",
                    "Ask about anything - work, learning, problems...",
                )
                .await?;

                if !query.trim().is_empty() {
                    suggestion_handler_v3::get_suggestions_streaming(&query).await;
                } else {
                    println!("{}", "ℹ️  No query entered, returning to menu.".yellow());
                }
                pause_for_user().await;
            }
            3 => {
                // View recent logs
                println!("\n{}", "📊 Recent Log Entries".cyan().bold());
                note_handler::show_recent_notes(5).await;
                pause_for_user().await;
            }
            4 => {
                // System & Configuration
                show_system_menu().await?;
            }
            5 => {
                // Help & Information
                show_help_menu().await?;
            }
            6 => {
                // Exit
                show_goodbye();
                break;
            }
            _ => {
                println!("{}", "Invalid selection. Please try again.".red());
                pause_for_user().await;
            }
        }
    }

    Ok(())
}

fn show_welcome() {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen
    println!(
        "{}",
        "╔══════════════════════════════════════╗".bright_blue()
    );
    println!(
        "{}",
        "║         Logswise CLI v1.0.0          ║".bright_blue()
    );
    println!(
        "{}",
        "║        Main Interface Started        ║".bright_blue()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════╝".bright_blue()
    );
    println!();
    println!(
        "{}",
        "Welcome to the unified Logswise CLI interface!".bright_white()
    );
    println!("• Chat with AI and get personalized suggestions");
    println!("• Log your thoughts, ideas, and experiences");
    println!("• Access all system features and help from one place");
    println!();
    println!("{}", "Press Enter to continue...".bright_black());

    let _ = io::stdin().read_line(&mut String::new());
}

fn show_main_menu() {
    println!("{}", "Logswise CLI - Main Interface".bright_cyan().bold());
    println!("{}", "━".repeat(50).bright_blue());
    println!();

    println!(
        "{}  {}",
        "1.".bright_white().bold(),
        "💬 Chat with AI".bright_white()
    );
    println!(
        "{}  {}",
        "2.".bright_white().bold(),
        "📝 Add new log entry".bright_white()
    );
    println!(
        "{}  {}",
        "3.".bright_white().bold(),
        "💡 Get AI suggestions".bright_white()
    );
    println!(
        "{}  {}",
        "4.".bright_white().bold(),
        "📊 View recent logs".bright_white()
    );
    println!(
        "{}  {}",
        "5.".bright_white().bold(),
        "⚙️  System & Configuration".bright_white()
    );
    println!(
        "{}  {}",
        "6.".bright_white().bold(),
        "❓ Help & Information".bright_white()
    );
    println!(
        "{}  {}",
        "7.".bright_white().bold(),
        "🚪 Exit".bright_white()
    );
    println!();
    println!(
        "{}",
        "Use arrow keys, numbers (1-7), or 'q' to quit".bright_black()
    );
    println!();
}

fn show_menu_selector() -> Result<usize, Box<dyn std::error::Error>> {
    let options = vec![
        "💬 Chat with AI",
        "📝 Add new log entry",
        "💡 Get AI suggestions",
        "📊 View recent logs",
        "⚙️  System & Configuration",
        "❓ Help & Information",
        "🚪 Exit",
    ];

    let selection = Select::new()
        .with_prompt("Select an option")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(selection)
}

async fn get_user_input(prompt: &str, placeholder: &str) -> Result<String, String> {
    println!();
    println!("{}", prompt.bright_cyan());
    println!("{}", format!("({placeholder})").bright_black());
    println!();

    let input: String = Input::new()
        .with_prompt("Enter your input")
        .allow_empty(true)
        .interact_text()
        .map_err(|e| format!("Input error: {e}"))?;

    Ok(input)
}

async fn pause_for_user() {
    println!();
    println!("{}", "Press Enter to return to menu...".bright_black());
    let _ = io::stdin().read_line(&mut String::new());
}

fn show_goodbye() {
    print!("\x1B[2J\x1B[1;1H"); // Clear screen
    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════╗".bright_green()
    );
    println!(
        "{}",
        "║    Thanks for using Logswise CLI!    ║".bright_green()
    );
    println!(
        "{}",
        "║         See you next time! 👋        ║".bright_green()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════╝".bright_green()
    );
    println!();
    println!(
        "{}",
        "Your logs and conversations are safely stored.".bright_white()
    );
    println!(
        "{}",
        "Run 'cargo run' anytime to return to this interface.".bright_black()
    );
    println!();
}

/// Enhanced interactive mode with keyboard shortcuts
#[allow(dead_code)]
pub async fn run_enhanced_interactive() -> Result<(), String> {
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::time::Duration;

    // Setup terminal for raw input
    enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {e}"))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| format!("Failed to setup terminal: {e}"))?;

    let mut selected_index = 0;
    let menu_items = vec![
        "💬 Chat with AI",
        "📝 Add new log entry",
        "💡 Get AI suggestions",
        "📊 View recent logs",
        "🚪 Exit",
    ];

    loop {
        // Clear screen and show menu
        print!("\x1B[2J\x1B[1;1H");
        show_enhanced_menu(selected_index, &menu_items);

        // Handle keyboard input
        if event::poll(Duration::from_millis(100)).map_err(|e| format!("Event poll error: {e}"))? {
            if let Event::Key(KeyEvent { code, .. }) =
                event::read().map_err(|e| format!("Key read error: {e}"))?
            {
                match code {
                    KeyCode::Up => {
                        selected_index = if selected_index == 0 {
                            menu_items.len() - 1
                        } else {
                            selected_index - 1
                        };
                    }
                    KeyCode::Down => {
                        selected_index = (selected_index + 1) % menu_items.len();
                    }
                    KeyCode::Enter => {
                        // Cleanup terminal before executing action
                        disable_raw_mode()
                            .map_err(|e| format!("Failed to disable raw mode: {e}"))?;
                        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)
                            .map_err(|e| format!("Failed to cleanup terminal: {e}"))?;

                        // Execute selected action
                        execute_menu_action(selected_index).await?;

                        if selected_index == 4 {
                            // Exit option
                            return Ok(());
                        }

                        // Re-setup terminal for continued use
                        enable_raw_mode()
                            .map_err(|e| format!("Failed to re-enable raw mode: {e}"))?;
                        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
                            .map_err(|e| format!("Failed to re-setup terminal: {e}"))?;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('1') => selected_index = 0,
                    KeyCode::Char('2') => selected_index = 1,
                    KeyCode::Char('3') => selected_index = 2,
                    KeyCode::Char('4') => selected_index = 3,
                    KeyCode::Char('5') => selected_index = 4,
                    _ => {}
                }
            }
        }
    }

    // Cleanup terminal
    disable_raw_mode().map_err(|e| format!("Failed to disable raw mode: {e}"))?;
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)
        .map_err(|e| format!("Failed to cleanup terminal: {e}"))?;

    show_goodbye();
    Ok(())
}

#[allow(dead_code)]
fn show_enhanced_menu(selected_index: usize, menu_items: &[&str]) {
    println!("{}", "Logswise CLI - Interactive Mode".bright_cyan().bold());
    println!("{}", "━".repeat(50).bright_blue());
    println!();

    for (index, item) in menu_items.iter().enumerate() {
        let number = format!("{}.", index + 1);
        if index == selected_index {
            println!(
                "{} {} {}",
                "❯".bright_cyan().bold(),
                number.bright_cyan().bold(),
                item.bright_cyan().bold()
            );
        } else {
            println!("  {} {}", number.bright_black(), item.bright_white());
        }
    }

    println!();
    println!(
        "{}",
        "Controls: ↑↓ Navigate | Enter Select | Numbers 1-5 | Q/ESC Quit".bright_black()
    );
}

#[allow(dead_code)]
async fn execute_menu_action(selection: usize) -> Result<(), String> {
    match selection {
        0 => {
            println!("\n{}", "💬 Starting AI Chat Session...".cyan().bold());
            if let Err(e) = chat_session::start_chat_session().await {
                println!("{}", format!("Chat error: {e}").red());
            }
        }
        1 => {
            println!("\n{}", "📝 Add New Log Entry".cyan().bold());
            let note_content = get_user_input(
                "What would you like to log?",
                "Type your thoughts, ideas, or notes here...",
            )
            .await?;

            if !note_content.trim().is_empty() {
                note_handler::add_note(&note_content).await;
                println!("{}", "✅ Log entry saved successfully!".green());
            } else {
                println!("{}", "ℹ️  No content entered.".yellow());
            }
            pause_for_user().await;
        }
        2 => {
            println!("\n{}", "💡 Get AI Suggestions".cyan().bold());
            let query = get_user_input(
                "What do you need suggestions for?",
                "Ask about anything - work, learning, problems...",
            )
            .await?;

            if !query.trim().is_empty() {
                suggestion_handler_v3::get_suggestions_streaming(&query).await;
            } else {
                println!("{}", "ℹ️  No query entered.".yellow());
            }
            pause_for_user().await;
        }
        3 => {
            println!("\n{}", "📊 Recent Log Entries".cyan().bold());
            note_handler::show_recent_notes(5).await;
            pause_for_user().await;
        }
        4 => {
            // Exit - handled by caller
        }
        _ => {
            println!("{}", "Invalid selection.".red());
            pause_for_user().await;
        }
    }

    Ok(())
}

async fn check_configuration() {
    if !config::config_exists() {
        println!(
            "{}",
            "⚠️  No configuration found. Please run setup first.".yellow()
        );
        println!(
            "{}",
            "You can access setup from the System & Configuration menu.".bright_black()
        );
        println!();
        pause_for_user().await;
    }
}

async fn show_system_menu() -> Result<(), String> {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "System & Configuration".bright_cyan().bold());
        println!("{}", "━".repeat(50).bright_blue());
        println!();

        let options = vec![
            "🔧 Run Setup",
            "🏥 System Health Check",
            "📊 Show Stats",
            "🎯 Personalization Settings",
            "🔄 Initialize Database",
            "🔙 Back to Main Menu",
        ];

        let selection = Select::new()
            .with_prompt("Select system option")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| format!("Menu selection error: {e}"))?;

        match selection {
            0 => {
                // Run Setup
                use crate::handlers::setup::SetupHandler;
                let mut setup_handler = SetupHandler::new();
                setup_handler
                    .run_smart_setup(false, false, false, None)
                    .await;
                pause_for_user().await;
            }
            1 => {
                // System Health Check
                let system_handler = SystemHandler::new();
                system_handler.run_doctor().await;
                pause_for_user().await;
            }
            2 => {
                // Show Stats
                let system_handler = SystemHandler::new();
                system_handler.print_stats();
                pause_for_user().await;
            }
            3 => {
                // Personalization
                show_personalization_menu().await?;
            }
            4 => {
                // Initialize Database
                let system_handler = SystemHandler::new();
                system_handler.run_init().await;
                pause_for_user().await;
            }
            5 => {
                // Back to main menu
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn show_help_menu() -> Result<(), String> {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "Help & Information".bright_cyan().bold());
        println!("{}", "━".repeat(50).bright_blue());
        println!();

        let options = vec![
            "ℹ️  About Logswise CLI",
            "📖 How Logswise Works",
            "🤖 About AI Models",
            "🔙 Back to Main Menu",
        ];

        let selection = Select::new()
            .with_prompt("Select help topic")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| format!("Menu selection error: {e}"))?;

        let help_handler = HelpHandler::new();

        match selection {
            0 => {
                help_handler.print_about();
                pause_for_user().await;
            }
            1 => {
                help_handler.print_how();
                pause_for_user().await;
            }
            2 => {
                help_handler.print_models();
                pause_for_user().await;
            }
            3 => {
                // Back to main menu
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

async fn show_personalization_menu() -> Result<(), String> {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "Personalization Settings".bright_cyan().bold());
        println!("{}", "━".repeat(50).bright_blue());
        println!();

        let options = vec![
            "🎯 Setup Personalization",
            "✏️  Update Settings",
            "👀 Show Current Settings",
            "💬 Provide Feedback",
            "🔙 Back to System Menu",
        ];

        let selection = Select::new()
            .with_prompt("Select personalization option")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| format!("Menu selection error: {e}"))?;

        use crate::handlers::personalization::PersonalizationHandler;
        let personalization_handler = PersonalizationHandler::new();

        match selection {
            0 => {
                personalization_handler.setup_personalization();
                pause_for_user().await;
            }
            1 => {
                personalization_handler.update_personalization();
                pause_for_user().await;
            }
            2 => {
                personalization_handler.show_personalization();
                pause_for_user().await;
            }
            3 => {
                let category =
                    get_user_input("Enter feedback category (optional):", "general").await?;
                let category_opt = if category.trim().is_empty() {
                    None
                } else {
                    Some(category)
                };
                personalization_handler.feedback(category_opt);
                pause_for_user().await;
            }
            4 => {
                // Back to system menu
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
