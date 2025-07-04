use colored::*;
use dialoguer::{Confirm, Input, Select};
use serde_json::{json, Value};

use crate::config::setup_modes::SetupOptions;
use crate::config::templates::ConfigTemplate;
use crate::types::SupabaseConfig;
use crate::utils::load_profile;

pub struct ConfigUpdater;

impl ConfigUpdater {
    pub async fn run_selective_update(templates: &mut Vec<ConfigTemplate>) -> Result<(), String> {
        println!("{}", "🔄 Selective Configuration Update".green().bold());

        let current_config = load_profile()?;

        let update_options = vec![
            "👤 Profile Info (profession, job title, company)",
            "💻 Technical Preferences (language, LLM, tools)",
            "🏢 Work Environment (company size, work mode)",
            "🔧 Technical Setup (Ollama URL, embedding model)",
            "🗄️  Database Settings (Supabase configuration)",
            "📋 Save as Template",
            "🔍 View Current Configuration",
        ];

        loop {
            let selection = Select::new()
                .with_prompt("What would you like to update?")
                .items(&update_options)
                .interact()
                .map_err(|e| format!("Input error: {e}"))?;

            match selection {
                0 => Self::update_profile_info(&current_config)?,
                1 => Self::update_technical_preferences(&current_config)?,
                2 => Self::update_work_environment(&current_config)?,
                3 => Self::update_technical_setup(&current_config)?,
                4 => Self::update_database_settings(&current_config).await?,
                5 => Self::save_current_as_template(templates, &current_config)?,
                6 => Self::show_current_configuration(&current_config)?,
                _ => {}
            }

            let continue_updating = Confirm::new()
                .with_prompt("Update another section?")
                .default(false)
                .interact()
                .map_err(|e| format!("Input error: {e}"))?;

            if !continue_updating {
                break;
            }
        }

        println!("{}", "✅ Configuration updated successfully!".green());
        Ok(())
    }

    fn update_profile_info(current_config: &Value) -> Result<(), String> {
        println!("{}", "👤 Updating Profile Information".yellow().bold());

        let mut updated_config = current_config.clone();
        let profession_options = SetupOptions::profession_options();

        let current_profession = current_config["profession"].as_str().unwrap_or("");
        let default_idx = profession_options
            .iter()
            .position(|&x| x == current_profession)
            .unwrap_or(0);

        let profession_idx = Select::new()
            .with_prompt(format!("Profession (current: {current_profession})"))
            .items(&profession_options)
            .default(default_idx)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["profession"] = json!(profession_options[profession_idx]);

        let job_title_options = SetupOptions::job_title_options();
        let current_job_title = current_config["jobTitle"].as_str().unwrap_or("");
        let default_idx = job_title_options
            .iter()
            .position(|&x| x == current_job_title)
            .unwrap_or(0);

        let job_title_idx = Select::new()
            .with_prompt(format!("Job Title (current: {current_job_title})"))
            .items(&job_title_options)
            .default(default_idx)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["jobTitle"] = json!(job_title_options[job_title_idx]);

        let current_company = current_config["companyName"].as_str().unwrap_or("");
        let company_name: String = Input::new()
            .with_prompt(format!("Company Name (current: {current_company})"))
            .default(current_company.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["companyName"] = json!(company_name);

        Self::save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_technical_preferences(current_config: &Value) -> Result<(), String> {
        println!("{}", "💻 Updating Technical Preferences".yellow().bold());

        let mut updated_config = current_config.clone();
        let language_options = SetupOptions::language_options();

        let current_language = current_config["preferredLanguage"].as_str().unwrap_or("");
        let default_idx = language_options
            .iter()
            .position(|&x| x == current_language)
            .unwrap_or(0);

        let language_idx = Select::new()
            .with_prompt(format!("Preferred Language (current: {current_language})"))
            .items(&language_options)
            .default(default_idx)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["preferredLanguage"] = json!(language_options[language_idx]);

        let current_llm = current_config["llmName"].as_str().unwrap_or("");
        let llm_name: String = Input::new()
            .with_prompt(format!("LLM Name (current: {current_llm})"))
            .default(current_llm.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["llmName"] = json!(llm_name);

        Self::save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_work_environment(current_config: &Value) -> Result<(), String> {
        println!("{}", "🏢 Updating Work Environment".yellow().bold());

        let mut updated_config = current_config.clone();
        let size_options = SetupOptions::company_size_options();
        let current_size = current_config["companySize"].as_str().unwrap_or("");
        let default_idx = size_options
            .iter()
            .position(|&x| x == current_size)
            .unwrap_or(0);

        let size_idx = Select::new()
            .with_prompt(format!("Company Size (current: {current_size})"))
            .items(&size_options)
            .default(default_idx)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["companySize"] = json!(size_options[size_idx]);

        let work_mode_options = SetupOptions::work_mode_options();
        let current_work_mode = current_config["workMode"].as_str().unwrap_or("");
        let default_idx = work_mode_options
            .iter()
            .position(|&x| x == current_work_mode)
            .unwrap_or(0);

        let work_mode_idx = Select::new()
            .with_prompt(format!("Work Mode (current: {current_work_mode})"))
            .items(&work_mode_options)
            .default(default_idx)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["workMode"] = json!(work_mode_options[work_mode_idx]);

        Self::save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_technical_setup(current_config: &Value) -> Result<(), String> {
        println!("{}", "🔧 Updating Technical Setup".yellow().bold());

        let mut updated_config = current_config.clone();

        let current_url = current_config["ollamaBaseUrl"]
            .as_str()
            .unwrap_or("http://localhost:11434");
        let ollama_url: String = Input::new()
            .with_prompt(format!("Ollama Base URL (current: {current_url})"))
            .default(current_url.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["ollamaBaseUrl"] = json!(ollama_url);

        let current_embedding = current_config["embeddingModel"]
            .as_str()
            .unwrap_or("nomic-embed-text");
        let embedding_model: String = Input::new()
            .with_prompt(format!("Embedding Model (current: {current_embedding})"))
            .default(current_embedding.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["embeddingModel"] = json!(embedding_model);

        Self::save_configuration(&updated_config)?;
        Ok(())
    }

    async fn update_database_settings(current_config: &Value) -> Result<(), String> {
        println!("{}", "🗄️  Updating Database Settings".yellow().bold());
        println!(
            "{}",
            "⚠️  This will test the new connection before saving".yellow()
        );

        let mut updated_config = current_config.clone();

        let current_url = current_config["supabaseUrl"].as_str().unwrap_or("");
        let supabase_url: String = Input::new()
            .with_prompt(format!("Supabase URL (current: {current_url})"))
            .default(current_url.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let current_key = current_config["supabaseApiKey"].as_str().unwrap_or("");
        let masked_key = if current_key.len() > 8 {
            format!(
                "{}...{}",
                &current_key[..4],
                &current_key[current_key.len() - 4..]
            )
        } else {
            "****".to_string()
        };

        let supabase_api_key: String = Input::new()
            .with_prompt(format!("Supabase API Key (current: {masked_key})"))
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let config = SupabaseConfig {
            project_url: supabase_url.clone(),
            api_key: supabase_api_key.clone(),
        };

        println!("{}", "Testing Supabase connection...".cyan());

        match crate::services::supabase::test_connection_async(&config).await {
            Ok(_) => {
                println!("{}", "✅ Connection successful!".green());
                updated_config["supabaseUrl"] = json!(supabase_url);
                updated_config["supabaseApiKey"] = json!(supabase_api_key);
                Self::save_configuration(&updated_config)?;
            }
            Err(e) => {
                println!("{}", format!("❌ Connection failed: {e}").red());
                println!("{}", "Configuration not updated.".yellow());
            }
        }

        Ok(())
    }

    fn save_current_as_template(
        templates: &mut Vec<ConfigTemplate>,
        current_config: &Value,
    ) -> Result<(), String> {
        println!(
            "{}",
            "📋 Save Current Configuration as Template".yellow().bold()
        );

        let template_name: String = Input::new()
            .with_prompt("Template name")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let template_description: String = Input::new()
            .with_prompt("Template description")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let template = ConfigTemplate {
            name: template_name,
            description: template_description,
            profession: current_config["profession"].as_str().map(|s| s.to_string()),
            job_title: current_config["jobTitle"].as_str().map(|s| s.to_string()),
            company_size: current_config["companySize"]
                .as_str()
                .map(|s| s.to_string()),
            years_experience: current_config["yearsExperience"]
                .as_str()
                .map(|s| s.to_string()),
            preferred_language: current_config["preferredLanguage"]
                .as_str()
                .map(|s| s.to_string()),
            work_mode: current_config["workMode"].as_str().map(|s| s.to_string()),
            llm_name: current_config["llmName"].as_str().map(|s| s.to_string()),
            ollama_base_url: current_config["ollamaBaseUrl"]
                .as_str()
                .map(|s| s.to_string()),
            embedding_model: current_config["embeddingModel"]
                .as_str()
                .map(|s| s.to_string()),
        };

        templates.push(template);

        println!("{}", "✅ Template saved successfully!".green());
        Ok(())
    }

    fn show_current_configuration(current_config: &Value) -> Result<(), String> {
        println!("{}", "🔍 Current Configuration".cyan().bold());
        println!("{}", "─".repeat(50).bright_black());

        let fields = vec![
            ("Profession", "profession"),
            ("Job Title", "jobTitle"),
            ("Company", "companyName"),
            ("Company Size", "companySize"),
            ("Experience", "yearsExperience"),
            ("Preferred Language", "preferredLanguage"),
            ("Work Mode", "workMode"),
            ("LLM Name", "llmName"),
            ("Ollama URL", "ollamaBaseUrl"),
            ("Embedding Model", "embeddingModel"),
        ];

        for (display_name, key) in fields {
            let value = current_config
                .get(key)
                .and_then(|v| v.as_str())
                .unwrap_or("Not set");
            println!("  {}: {}", display_name.bright_white(), value.yellow());
        }
        println!();
        Ok(())
    }

    fn save_configuration(config: &Value) -> Result<(), String> {
        use dirs::home_dir;
        use std::fs;

        let mut setup_path = home_dir().ok_or("Could not determine home directory")?;
        setup_path.push(".logswise");
        fs::create_dir_all(&setup_path).map_err(|e| format!("Failed to create directory: {e}"))?;
        setup_path.push("setup.json");

        fs::write(&setup_path, serde_json::to_string_pretty(config).unwrap())
            .map_err(|e| format!("Failed to write configuration: {e}"))?;

        Ok(())
    }
}
