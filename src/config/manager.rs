use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

use crate::config::setup_modes::{SetupAction, SetupMode, SetupOptions};
use crate::config::templates::ConfigTemplate;
use crate::config::updater::ConfigUpdater;
use crate::config::validator::ConfigValidator;
use crate::types::SupabaseConfig;
use crate::utils::load_profile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigManager {
    pub templates: Vec<ConfigTemplate>,
    pub last_used_values: HashMap<String, String>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            templates: ConfigTemplate::default_templates(),
            last_used_values: HashMap::new(),
        }
    }

    pub fn load_or_create() -> Result<Self, String> {
        let mut path = home_dir().ok_or("Could not determine home directory")?;
        path.push(".logswise/config_manager.json");

        if path.exists() {
            let data = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read config manager: {e}"))?;
            let mut manager: Self = serde_json::from_str(&data)
                .map_err(|e| format!("Failed to parse config manager: {e}"))?;

            manager.ensure_default_templates();
            Ok(manager)
        } else {
            let manager = Self::new();
            manager.save()?;
            Ok(manager)
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let mut path = home_dir().ok_or("Could not determine home directory")?;
        path.push(".logswise");
        fs::create_dir_all(&path).map_err(|e| format!("Failed to create directory: {e}"))?;
        path.push("config_manager.json");

        let data = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config manager: {e}"))?;
        fs::write(&path, data).map_err(|e| format!("Failed to write config manager: {e}"))?;
        Ok(())
    }

    fn ensure_default_templates(&mut self) {
        let default_templates = ConfigTemplate::default_templates();
        for default_template in default_templates {
            if !self
                .templates
                .iter()
                .any(|t| t.name == default_template.name)
            {
                self.templates.push(default_template);
            }
        }
    }

    pub async fn run_smart_setup(&mut self) -> Result<(), String> {
        println!("{}", "🚀 Smart Setup - Logswise CLI".cyan().bold());
        println!("Configure everything you want with intelligent defaults and presets!\n");

        let existing_config = self.check_existing_setup();

        if existing_config.is_ok() {
            let action = self.choose_setup_action()?;
            match action {
                SetupAction::UpdateExisting => {
                    return ConfigUpdater::run_selective_update(&mut self.templates).await
                }
                SetupAction::CreateNew => {
                    println!("{}", "Creating new configuration...\n".yellow());
                }
                SetupAction::UseTemplate => {
                    return self.setup_from_template().await;
                }
            }
        }

        let setup_mode = self.choose_setup_mode()?;

        match setup_mode {
            SetupMode::Express => self.run_express_setup().await,
            SetupMode::Template => self.setup_from_template().await,
            SetupMode::Custom => self.run_custom_setup().await,
            SetupMode::Import => self.import_existing_setup(),
        }
    }

    fn check_existing_setup(&self) -> Result<Value, String> {
        load_profile()
    }

    fn choose_setup_action(&self) -> Result<SetupAction, String> {
        let options = vec![
            "Update existing configuration",
            "Create new configuration",
            "Use a template/preset",
        ];

        let selection = Select::new()
            .with_prompt("Existing configuration found. What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        match selection {
            0 => Ok(SetupAction::UpdateExisting),
            1 => Ok(SetupAction::CreateNew),
            2 => Ok(SetupAction::UseTemplate),
            _ => Err("Invalid selection".to_string()),
        }
    }

    fn choose_setup_mode(&self) -> Result<SetupMode, String> {
        println!("{}", "Choose your setup mode:".yellow().bold());

        let options = vec![
            "🚀 Express Setup (5 questions, smart defaults)",
            "📋 Template/Preset (choose from predefined configurations)",
            "⚙️  Custom Setup (full configuration control)",
            "📁 Import Existing (from another machine)",
        ];

        let selection = Select::new()
            .with_prompt("How would you like to set up Logswise CLI?")
            .items(&options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        match selection {
            0 => Ok(SetupMode::Express),
            1 => Ok(SetupMode::Template),
            2 => Ok(SetupMode::Custom),
            3 => Ok(SetupMode::Import),
            _ => Err("Invalid selection".to_string()),
        }
    }

    pub async fn run_express_setup(&mut self) -> Result<(), String> {
        println!("{}", "🚀 Express Setup".green().bold());
        println!("Just 5 questions to get you started!\n");

        let role_options = SetupOptions::role_options();
        let role_idx = Select::new()
            .with_prompt("What's your primary role?")
            .items(&role_options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let tech_options = SetupOptions::tech_options();
        let tech_idx = Select::new()
            .with_prompt("What's your primary programming language?")
            .items(&tech_options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let company_name: String = Input::new()
            .with_prompt("Company name (optional, press Enter to skip)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let llm_options = SetupOptions::llm_options();
        let llm_idx = Select::new()
            .with_prompt("Which LLM would you like to use?")
            .items(&llm_options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let llm_name = match llm_idx {
            0 => "llama3.2".to_string(),
            1 => "codellama".to_string(),
            2 => "mistral".to_string(),
            3 => Input::new()
                .with_prompt("Enter your LLM name")
                .interact_text()
                .map_err(|e| format!("Input error: {e}"))?,
            _ => "llama3.2".to_string(),
        };

        let (supabase_url, supabase_api_key) = self.setup_supabase_smart().await?;

        let (job_title, company_size, years_experience, work_mode) =
            SetupOptions::get_smart_defaults(role_options[role_idx]);

        let profile_data = json!({
            "profession": role_options[role_idx],
            "jobTitle": job_title,
            "companyName": if company_name.is_empty() { "My Company" } else { &company_name },
            "companySize": company_size,
            "yearsExperience": years_experience,
            "preferredLanguage": tech_options[tech_idx],
            "workMode": work_mode,
            "llmName": llm_name,
            "ollamaBaseUrl": "http://localhost:11434",
            "embeddingModel": "nomic-embed-text",
            "supabaseUrl": supabase_url,
            "supabaseApiKey": supabase_api_key
        });

        self.save_configuration(&profile_data)?;

        println!(
            "{}",
            "✅ Express setup complete! You're ready to use Logswise CLI!"
                .green()
                .bold()
        );
        println!(
            "{}",
            "💡 You can update any settings later with 'logswise-cli setup --update'".cyan()
        );

        Ok(())
    }

    pub async fn setup_from_template(&mut self) -> Result<(), String> {
        println!("{}", "📋 Template/Preset Setup".green().bold());
        println!("Choose from predefined configurations:\n");

        let template_options: Vec<String> = self
            .templates
            .iter()
            .map(|t| format!("{} - {}", t.name, t.description))
            .collect();

        let template_idx = Select::new()
            .with_prompt("Choose a template")
            .items(&template_options)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let template = &self.templates[template_idx].clone();

        println!("\n{}", format!("Using template: {}", template.name).cyan());

        let customize = Confirm::new()
            .with_prompt("Would you like to customize this template?")
            .default(false)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let mut config_data = self.template_to_config(template);

        if customize {
            config_data = self.customize_template_config(config_data)?;
        }

        let company_name: String = Input::new()
            .with_prompt("Enter your company name")
            .default("My Company".to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        config_data["companyName"] = json!(company_name);

        let (supabase_url, supabase_api_key) = self.setup_supabase_smart().await?;
        config_data["supabaseUrl"] = json!(supabase_url);
        config_data["supabaseApiKey"] = json!(supabase_api_key);

        self.save_configuration(&config_data)?;

        println!("{}", "✅ Template setup complete!".green().bold());
        Ok(())
    }

    fn template_to_config(&self, template: &ConfigTemplate) -> Value {
        json!({
            "profession": template.profession.as_ref().unwrap_or(&"Software Developer".to_string()),
            "jobTitle": template.job_title.as_ref().unwrap_or(&"Mid".to_string()),
            "companyName": "My Company",
            "companySize": template.company_size.as_ref().unwrap_or(&"10-100".to_string()),
            "yearsExperience": template.years_experience.as_ref().unwrap_or(&"3-5 years".to_string()),
            "preferredLanguage": template.preferred_language.as_ref().unwrap_or(&"JavaScript/TypeScript".to_string()),
            "workMode": template.work_mode.as_ref().unwrap_or(&"Remote".to_string()),
            "llmName": template.llm_name.as_ref().unwrap_or(&"llama3.2".to_string()),
            "ollamaBaseUrl": template.ollama_base_url.as_ref().unwrap_or(&"http://localhost:11434".to_string()),
            "embeddingModel": template.embedding_model.as_ref().unwrap_or(&"nomic-embed-text".to_string()),
        })
    }

    fn customize_template_config(&self, mut config: Value) -> Result<Value, String> {
        println!("{}", "Customizing template...".yellow());

        let fields_to_customize = [
            ("Job Title", "jobTitle"),
            ("Company Size", "companySize"),
            ("Years Experience", "yearsExperience"),
            ("Programming Language", "preferredLanguage"),
            ("Work Mode", "workMode"),
            ("LLM Name", "llmName"),
        ];

        let customize_indices = MultiSelect::new()
            .with_prompt("Which fields would you like to customize? (Select multiple)")
            .items(
                &fields_to_customize
                    .iter()
                    .map(|(name, _)| name)
                    .collect::<Vec<_>>(),
            )
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        for &idx in &customize_indices {
            let (field_name, field_key) = &fields_to_customize[idx];
            let current_value = config[field_key].as_str().unwrap_or("");

            match *field_key {
                "jobTitle" => {
                    let options = SetupOptions::job_title_options();
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "companySize" => {
                    let options = SetupOptions::company_size_options();
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "yearsExperience" => {
                    let options = SetupOptions::years_experience_options();
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "preferredLanguage" => {
                    let options = SetupOptions::language_options();
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "workMode" => {
                    let options = SetupOptions::work_mode_options();
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "llmName" => {
                    let new_value: String = Input::new()
                        .with_prompt(format!("Enter {field_name} (current: {current_value})"))
                        .default(current_value.to_string())
                        .interact_text()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(new_value);
                }
                _ => {}
            }
        }

        Ok(config)
    }

    async fn run_custom_setup(&mut self) -> Result<(), String> {
        println!("{}", "⚙️  Custom Setup".green().bold());
        println!("Full control over your configuration\n");

        // TODO: Implement setup functionality
        println!("Setup functionality placeholder");
        Ok(())
    }

    pub fn import_existing_setup(&mut self) -> Result<(), String> {
        println!("{}", "📁 Import Existing Setup".green().bold());

        let import_path: String = Input::new()
            .with_prompt("Enter path to existing setup.json file")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let data =
            fs::read_to_string(&import_path).map_err(|e| format!("Failed to read file: {e}"))?;

        let config: Value =
            serde_json::from_str(&data).map_err(|e| format!("Failed to parse JSON: {e}"))?;

        ConfigValidator::validate_import_config(&config)?;

        self.save_configuration(&config)?;

        println!("{}", "✅ Configuration imported successfully!".green());
        Ok(())
    }

    async fn setup_supabase_smart(&self) -> Result<(String, String), String> {
        println!("{}", "🗄️  Supabase Configuration".cyan().bold());

        let supabase_url: String = Input::new()
            .with_prompt("Supabase Project URL")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let supabase_api_key: String = Input::new()
            .with_prompt("Supabase API Key")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let config = SupabaseConfig {
            project_url: supabase_url.clone(),
            api_key: supabase_api_key.clone(),
        };

        println!("{}", "Testing connection...".cyan());

        match crate::services::supabase::test_connection_async(&config).await {
            Ok(_) => println!("{}", "✅ Connection successful!".green()),
            Err(e) => {
                println!("{}", format!("❌ Connection failed: {e}").red());
                let continue_anyway = Confirm::new()
                    .with_prompt("Continue anyway? (You can fix this later)")
                    .default(false)
                    .interact()
                    .map_err(|e| format!("Input error: {e}"))?;

                if !continue_anyway {
                    return Err("Setup cancelled".to_string());
                }
            }
        }

        match crate::services::supabase::check_notes_table_exists_async(&config).await {
            Ok(false) => {
                let auto_setup = Confirm::new()
                    .with_prompt("Set up database tables automatically?")
                    .default(true)
                    .interact()
                    .map_err(|e| format!("Input error: {e}"))?;

                if auto_setup {
                    match crate::services::supabase::setup_database_schema_async(&config).await {
                        Ok(_) => println!("{}", "✅ Database setup complete!".green()),
                        Err(e) => {
                            println!("{}", format!("⚠️  Database setup failed: {e}").yellow())
                        }
                    }
                }
            }
            Ok(true) => println!("{}", "✅ Database already configured!".green()),
            Err(e) => println!("{}", format!("⚠️  Could not check database: {e}").yellow()),
        }

        Ok((supabase_url, supabase_api_key))
    }

    fn save_configuration(&self, config: &Value) -> Result<(), String> {
        let mut setup_path = home_dir().ok_or("Could not determine home directory")?;
        setup_path.push(".logswise");
        fs::create_dir_all(&setup_path).map_err(|e| format!("Failed to create directory: {e}"))?;
        setup_path.push("setup.json");

        fs::write(&setup_path, serde_json::to_string_pretty(config).unwrap())
            .map_err(|e| format!("Failed to write configuration: {e}"))?;

        Ok(())
    }
}
