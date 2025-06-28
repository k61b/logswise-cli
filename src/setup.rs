use colored::*;
use dialoguer::{Confirm, Input, Select};
use dirs::home_dir;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

// Import our Supabase service functions
use crate::services::supabase::{check_notes_table_exists, setup_database_schema, test_connection};
use crate::types::SupabaseConfig;

/// Runs the interactive setup process for user profile and configuration.
pub fn run_setup() {
    let profession_options = vec![
        "Software Developer",
        "Product Manager",
        "Designer",
        "Data Scientist",
        "QA Engineer",
        "DevOps Engineer",
        "Sales Engineer",
        "Technical Writer",
        "Other",
    ];
    let job_title_options = vec![
        "Intern", "Junior", "Mid", "Senior", "Lead", "Manager", "Director", "VP", "C-level",
        "Other",
    ];
    let company_size_options = vec![
        "1-10",
        "10-100",
        "100-500",
        "500-1000",
        "1000-5000",
        "5000+",
    ];
    let experience_options = vec![
        "<1 year",
        "1-3 years",
        "3-5 years",
        "5-10 years",
        "10+ years",
    ];
    let language_options = vec![
        "Rust",
        "Python",
        "JavaScript/TypeScript",
        "Go",
        "Java",
        "C#",
        "C/C++",
        "Ruby",
        "Swift",
        "Kotlin",
        "Other",
    ];
    let work_mode_options = vec!["Remote", "On-site", "Hybrid"];

    let profession = Select::new()
        .with_prompt("Select your profession:")
        .items(&profession_options)
        .default(0)
        .interact()
        .unwrap();
    let job_title = Select::new()
        .with_prompt("Select your job title:")
        .items(&job_title_options)
        .default(0)
        .interact()
        .unwrap();
    let company_name: String = Input::new()
        .with_prompt("Enter your company name:")
        .interact_text()
        .unwrap();
    let company_size = Select::new()
        .with_prompt("Select your company size:")
        .items(&company_size_options)
        .default(0)
        .interact()
        .unwrap();
    let years_experience = Select::new()
        .with_prompt("How many years of professional experience do you have?")
        .items(&experience_options)
        .default(0)
        .interact()
        .unwrap();
    let preferred_language = Select::new()
        .with_prompt("What is your preferred programming language?")
        .items(&language_options)
        .default(0)
        .interact()
        .unwrap();
    let work_mode = Select::new()
        .with_prompt("What is your preferred work mode?")
        .items(&work_mode_options)
        .default(0)
        .interact()
        .unwrap();
    let llm_name: String = Input::new()
        .with_prompt("Enter the LLM name (e.g., ollama, llama.cpp):")
        .interact_text()
        .unwrap();
    let ollama_url: String = Input::new()
        .with_prompt("Enter the Ollama base URL (default: http://localhost:11434):")
        .default("http://localhost:11434".to_string())
        .interact_text()
        .unwrap();
    let ollama_embedding_model: String = Input::new()
        .with_prompt("Enter the embedding model name (default: nomic-embed-text):")
        .default("nomic-embed-text".to_string())
        .interact_text()
        .unwrap();

    // Supabase setup with validation and automatic database setup
    let (supabase_url, supabase_api_key) = setup_supabase_with_validation();

    let profile_data = json!({
        "profession": profession_options[profession],
        "jobTitle": job_title_options[job_title],
        "companyName": company_name,
        "companySize": company_size_options[company_size],
        "yearsExperience": experience_options[years_experience],
        "preferredLanguage": language_options[preferred_language],
        "workMode": work_mode_options[work_mode],
        "llmName": llm_name,
        "ollamaBaseUrl": ollama_url,
        "embeddingModel": ollama_embedding_model,
        "supabaseUrl": supabase_url,
        "supabaseApiKey": supabase_api_key
    });

    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise");
    fs::create_dir_all(&setup_path).unwrap();
    setup_path.push("setup.json");
    fs::write(
        &setup_path,
        serde_json::to_string_pretty(&profile_data).unwrap(),
    )
    .unwrap();
    println!(
        "{}",
        "✅ Setup complete! You are ready to use Logswise CLI!".green()
    );

    // Ask if user wants to define an alias
    let define_alias = dialoguer::Confirm::new()
        .with_prompt(
            "Would you like to define 'logswise' as a shortcut for 'logswise-cli'? (recommended)",
        )
        .default(true)
        .interact()
        .unwrap();
    if define_alias {
        // Try to detect shell and append alias to the appropriate rc file
        let shell = std::env::var("SHELL").unwrap_or_default();
        let home = home_dir().unwrap_or(PathBuf::from("~"));
        let rc_file = if shell.contains("zsh") {
            home.join(".zshrc")
        } else if shell.contains("bash") {
            home.join(".bashrc")
        } else {
            home.join(".profile")
        };
        let alias_line = "alias logswise='logswise-cli'\n";
        let append_result = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&rc_file)
            .and_then(|mut file| std::io::Write::write_all(&mut file, alias_line.as_bytes()));
        match append_result {
            Ok(_) => println!("{} Alias 'logswise' added to {:?}. You may need to restart your terminal or run 'source {:?}' to use it.", "✅".green(), rc_file, rc_file),
            Err(e) => println!("{} Could not add alias automatically: {}\nYou can manually add this line to your shell config:\n{}", "⚠️".yellow(), e, alias_line),
        }
    }
}

/// Sets up Supabase configuration with validation and automatic database schema setup.
fn setup_supabase_with_validation() -> (String, String) {
    let supabase_url: String = Input::new()
        .with_prompt("Enter your Supabase project URL:")
        .interact_text()
        .unwrap();
    let supabase_api_key: String = Input::new()
        .with_prompt("Enter your Supabase API key:")
        .interact_text()
        .unwrap();

    // Create Supabase config
    let config = SupabaseConfig {
        project_url: supabase_url.clone(),
        api_key: supabase_api_key.clone(),
    };

    // Create progress spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Test Supabase connection
    let client = Client::new();
    spinner.set_message("Testing Supabase connection...");

    let test_result = test_connection(&client, &config);

    match test_result {
        Ok(_) => {
            spinner.finish_and_clear();
            println!("{} Supabase connection successful!", "✅".green());
        }
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("{} Supabase connection failed: {}", "❌".red(), e);
            eprintln!("Please check your Supabase URL and API key and try again.");
            std::process::exit(1);
        }
    }

    // Check if notes table exists
    spinner.set_message("Checking database schema...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let table_exists = check_notes_table_exists(&client, &config);

    match table_exists {
        Ok(true) => {
            spinner.finish_and_clear();
            println!("{} Database tables already exist.", "✅".green());
        }
        Ok(false) => {
            spinner.finish_and_clear();
            println!(
                "{} Notes table not found. Setting up database schema...",
                "⚠️".yellow()
            );

            // Ask user if they want automatic setup
            let auto_setup = Confirm::new()
                .with_prompt("Would you like to automatically set up the required database tables and schema?")
                .default(true)
                .interact()
                .unwrap();

            if auto_setup {
                // Setup database schema
                spinner.set_message("Creating database tables and schema...");
                spinner.enable_steady_tick(Duration::from_millis(100));

                let schema_setup_result = setup_database_schema(&client, &config);

                match schema_setup_result {
                    Ok(_) => {
                        spinner.finish_and_clear();
                        println!("{} Database schema set up successfully!", "✅".green());
                        println!("{}", "Your Supabase database is now ready for use.".cyan());
                    }
                    Err(e) => {
                        spinner.finish_and_clear();
                        println!("{} Automatic database setup failed: {}", "⚠️".yellow(), e);
                        println!();
                        println!(
                            "{}",
                            "Please set up your database manually using the following steps:"
                                .cyan()
                        );
                        println!("{}", "1. Go to your Supabase SQL Editor".bright_white());
                        println!("{}", "2. Run the commands shown above".bright_white());
                        println!(
                            "{}",
                            "3. Or follow the instructions in SUPABASE_SETUP.md".bright_white()
                        );
                        println!();

                        let continue_anyway = Confirm::new()
                            .with_prompt(
                                "Continue with setup anyway? (You can set up the database later)",
                            )
                            .default(true)
                            .interact()
                            .unwrap();

                        if !continue_anyway {
                            std::process::exit(1);
                        }
                    }
                }
            } else {
                println!("{}", "Skipping automatic database setup.".yellow());
                println!(
                    "{}",
                    "Please refer to SUPABASE_SETUP.md for manual setup instructions.".cyan()
                );
            }
        }
        Err(e) => {
            spinner.finish_and_clear();
            eprintln!("{} Error checking database: {}", "❌".red(), e);
            eprintln!("Please check your Supabase configuration and try again.");
            std::process::exit(1);
        }
    }

    (supabase_url, supabase_api_key)
}
