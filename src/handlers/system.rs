use clap::CommandFactory;
use clap_complete::{generate, Shell};
use colored::*;
use std::io;

pub struct SystemHandler {}

impl SystemHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_stats(&self) {
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
                println!("{}", format!("Error loading profile: {e}").red());
                println!("No profile found. Run 'logswise-cli setup' first.");
            }
        }
        println!(
            "\nNote: For full stats (note count, etc.), future versions will fetch from Supabase.\n"
        );
    }

    pub fn generate_completions(&self, shell: &str) {
        let shell_type = match shell.to_lowercase().as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "powershell" => Shell::PowerShell,
            _ => {
                println!(
                    "{}",
                    "❌ Unsupported shell. Available: bash, zsh, fish, powershell".red()
                );
                return;
            }
        };

        let mut app = crate::cli::Cli::command();
        generate(shell_type, &mut app, "logswise-cli", &mut io::stdout());
    }

    pub fn run_doctor(&self) {
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
                            println!("  ✅ {display_name} configured: {value}");
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
            let test_url = format!("{ollama_base_url}/api/tags");

            match reqwest::blocking::get(&test_url) {
                Ok(response) if response.status().is_success() => {
                    println!("  ✅ Ollama server is reachable");

                    // Try to test embedding model
                    let embedding_model = profile["embeddingModel"]
                        .as_str()
                        .unwrap_or("nomic-embed-text");
                    println!("  🔍 Testing embedding model: {}", embedding_model.cyan());

                    let client = reqwest::blocking::Client::new();
                    let embedding_url = format!("{ollama_base_url}/api/embeddings");
                    match crate::services::ollama::generate_embedding(
                        &client,
                        &embedding_url,
                        embedding_model,
                        "test",
                    ) {
                        Ok(_) => println!("  ✅ Embedding model '{embedding_model}' is working"),
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
                        let generate_url = format!("{ollama_base_url}/api/generate");
                        match crate::services::ollama::generate_suggestion(
                            &client,
                            &generate_url,
                            llm_name,
                            "test",
                        ) {
                            Ok(_) => println!("  ✅ LLM '{llm_name}' is working"),
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

            let client = reqwest::blocking::Client::new();

            // Test basic connection
            match crate::services::supabase::test_connection(&client, &supabase_config) {
                Ok(_) => {
                    println!("  ✅ Supabase connection successful");

                    // Test database schema
                    println!("  🔍 Checking database schema...");
                    match crate::services::supabase::check_notes_table_exists(
                        &client,
                        &supabase_config,
                    ) {
                        Ok(true) => {
                            println!("  ✅ Notes table exists and is accessible");

                            // Test if we can write to the table
                            println!("  🔍 Testing database write access...");
                            let test_url = format!("{}/rest/v1/notes", supabase_config.project_url);

                            match client
                                .post(&test_url)
                                .header("apikey", &supabase_config.api_key)
                                .header(
                                    "Authorization",
                                    format!("Bearer {}", &supabase_config.api_key),
                                )
                                .header("Content-Type", "application/json")
                                .header("Prefer", "return=minimal")
                                .json(&serde_json::json!({
                                    "content": "Doctor health check test note",
                                    "embedding": serde_json::Value::Null
                                }))
                                .send()
                            {
                                Ok(resp) if resp.status().is_success() => {
                                    println!("  ✅ Database write access confirmed");
                                }
                                Ok(resp) => {
                                    let status = resp.status();
                                    println!(
                                        "  ⚠️  Database write test failed: HTTP {}",
                                        status.to_string().yellow()
                                    );
                                    if status.as_u16() == 403 {
                                        println!(
                                            "      Check your RLS (Row Level Security) policies"
                                        );
                                    }
                                    issues_found += 1;
                                }
                                Err(e) => {
                                    println!(
                                        "  ⚠️  Database write test failed: {}",
                                        e.to_string().yellow()
                                    );
                                    issues_found += 1;
                                }
                            }
                        }
                        Ok(false) => {
                            println!("  ❌ Notes table does not exist");
                            println!("      You need to set up your database schema");

                            // Offer to set up database automatically
                            println!("\n{}", "🔧 Database Setup Available".bold().cyan());
                            println!("  The notes table is missing from your Supabase database.");
                            println!("  You can set up the database schema by running:");
                            println!("    {}", "logswise-cli setup".green());
                            println!("  Or set up manually using SUPABASE_SETUP.md");

                            issues_found += 1;
                        }
                        Err(e) => {
                            println!("  ❌ Failed to check database schema: {}", e.red());
                            println!("      This might indicate permission or connection issues");
                            issues_found += 1;
                        }
                    }
                }
                Err(e) => {
                    println!("  ❌ Supabase connection failed: {}", e.red());
                    println!("      Check your Supabase URL and API key");
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

    pub fn run_init(&self) {
        println!("\n{}\n", "🔧 Database Initialization".bold().cyan());

        // Check if we have Supabase configuration
        let supabase_config = match crate::utils::load_supabase_config() {
            Ok(config) => config,
            Err(_) => {
                println!("{}", "❌ No Supabase configuration found.".red());
                println!(
                    "Please run 'logswise-cli setup' first to configure your Supabase connection."
                );
                std::process::exit(1);
            }
        };

        println!("Found Supabase configuration:");
        println!("  Project URL: {}", supabase_config.project_url.cyan());
        println!(
            "  API Key: {}...{}",
            &supabase_config.api_key[..8].cyan(),
            &supabase_config.api_key[supabase_config.api_key.len() - 4..].cyan()
        );
        println!();

        let client = reqwest::blocking::Client::new();

        // Test connection
        println!("🔍 Testing Supabase connection...");
        match crate::services::supabase::test_connection(&client, &supabase_config) {
            Ok(_) => println!("  ✅ Connection successful!"),
            Err(e) => {
                println!("  ❌ Connection failed: {}", e.red());
                println!("Please check your Supabase URL and API key.");
                std::process::exit(1);
            }
        }

        // Check if database is already set up
        println!("\n🔍 Checking database schema...");
        match crate::services::supabase::check_notes_table_exists(&client, &supabase_config) {
            Ok(true) => {
                println!("  ✅ Notes table already exists!");
                println!(
                    "\n{}",
                    "Your database appears to be set up correctly.".green()
                );
                println!("Run 'logswise-cli doctor' for a comprehensive health check.");
            }
            Ok(false) => {
                println!("  ❌ Notes table not found.");
                println!("\n{}", "Setting up database schema...".cyan());

                match crate::services::supabase::setup_database_schema(&client, &supabase_config) {
                    Ok(_) => {
                        println!(
                            "\n{}",
                            "✅ Database initialization completed successfully!".green()
                        );
                        println!("Your Supabase database is now ready to use with Logswise CLI.");
                    }
                    Err(e) => {
                        println!(
                            "\n{}",
                            "⚠️  Automatic setup was not completely successful.".yellow()
                        );
                        println!("Error: {e}");
                        println!();
                        println!("Please complete the setup manually by following the instructions above,");
                        println!("or refer to the SUPABASE_SETUP.md file for detailed setup instructions.");
                    }
                }
            }
            Err(e) => {
                println!("  ❌ Failed to check database: {}", e.red());
                println!("Please check your Supabase configuration and permissions.");
                std::process::exit(1);
            }
        }

        println!("\n{}", "Next steps:".bold());
        println!("  • Run 'logswise-cli doctor' to verify everything is working");
        println!("  • Add your first note: 'logswise-cli note \"Your note here\"'");
        println!("  • Get suggestions: 'logswise-cli suggestion \"Your query\"'");
        println!("  • Start interactive mode: 'logswise-cli interactive'");
    }
}
