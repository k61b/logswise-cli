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
                println!("{}", format!("Error loading profile: {}", e).red());
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
}
