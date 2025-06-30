use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

use crate::services::supabase::{check_notes_table_exists, setup_database_schema, test_connection};
use crate::types::SupabaseConfig;
use crate::utils::load_profile;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigTemplate {
    pub name: String,
    pub description: String,
    pub profession: Option<String>,
    pub job_title: Option<String>,
    pub company_size: Option<String>,
    pub years_experience: Option<String>,
    pub preferred_language: Option<String>,
    pub work_mode: Option<String>,
    pub llm_name: Option<String>,
    pub ollama_base_url: Option<String>,
    pub embedding_model: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigManager {
    pub templates: Vec<ConfigTemplate>,
    pub last_used_values: HashMap<String, String>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            templates: Self::default_templates(),
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

            // Ensure we have default templates
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

    fn default_templates() -> Vec<ConfigTemplate> {
        vec![
            ConfigTemplate {
                name: "Software Developer - Startup".to_string(),
                description: "Full-stack developer at a startup company".to_string(),
                profession: Some("Software Developer".to_string()),
                job_title: Some("Senior".to_string()),
                company_size: Some("1-10".to_string()),
                years_experience: Some("3-5 years".to_string()),
                preferred_language: Some("JavaScript/TypeScript".to_string()),
                work_mode: Some("Remote".to_string()),
                llm_name: Some("llama3.2".to_string()),
                ollama_base_url: Some("http://localhost:11434".to_string()),
                embedding_model: Some("nomic-embed-text".to_string()),
            },
            ConfigTemplate {
                name: "Software Developer - Enterprise".to_string(),
                description: "Backend developer at a large enterprise".to_string(),
                profession: Some("Software Developer".to_string()),
                job_title: Some("Senior".to_string()),
                company_size: Some("1000-5000".to_string()),
                years_experience: Some("5-10 years".to_string()),
                preferred_language: Some("Java".to_string()),
                work_mode: Some("Hybrid".to_string()),
                llm_name: Some("llama3.2".to_string()),
                ollama_base_url: Some("http://localhost:11434".to_string()),
                embedding_model: Some("nomic-embed-text".to_string()),
            },
            ConfigTemplate {
                name: "Rust Developer".to_string(),
                description: "Systems programmer focused on Rust".to_string(),
                profession: Some("Software Developer".to_string()),
                job_title: Some("Mid".to_string()),
                company_size: Some("100-500".to_string()),
                years_experience: Some("3-5 years".to_string()),
                preferred_language: Some("Rust".to_string()),
                work_mode: Some("Remote".to_string()),
                llm_name: Some("codellama".to_string()),
                ollama_base_url: Some("http://localhost:11434".to_string()),
                embedding_model: Some("nomic-embed-text".to_string()),
            },
            ConfigTemplate {
                name: "Data Scientist".to_string(),
                description: "Machine learning and data analysis specialist".to_string(),
                profession: Some("Data Scientist".to_string()),
                job_title: Some("Senior".to_string()),
                company_size: Some("500-1000".to_string()),
                years_experience: Some("3-5 years".to_string()),
                preferred_language: Some("Python".to_string()),
                work_mode: Some("Hybrid".to_string()),
                llm_name: Some("llama3.2".to_string()),
                ollama_base_url: Some("http://localhost:11434".to_string()),
                embedding_model: Some("nomic-embed-text".to_string()),
            },
            ConfigTemplate {
                name: "DevOps Engineer".to_string(),
                description: "Infrastructure and deployment specialist".to_string(),
                profession: Some("DevOps Engineer".to_string()),
                job_title: Some("Senior".to_string()),
                company_size: Some("100-500".to_string()),
                years_experience: Some("5-10 years".to_string()),
                preferred_language: Some("Go".to_string()),
                work_mode: Some("Remote".to_string()),
                llm_name: Some("llama3.2".to_string()),
                ollama_base_url: Some("http://localhost:11434".to_string()),
                embedding_model: Some("nomic-embed-text".to_string()),
            },
            ConfigTemplate {
                name: "Quick Start - Minimal".to_string(),
                description: "Minimal configuration to get started quickly".to_string(),
                profession: Some("Software Developer".to_string()),
                job_title: Some("Mid".to_string()),
                company_size: Some("10-100".to_string()),
                years_experience: Some("3-5 years".to_string()),
                preferred_language: Some("JavaScript/TypeScript".to_string()),
                work_mode: Some("Remote".to_string()),
                llm_name: Some("llama3.2".to_string()),
                ollama_base_url: Some("http://localhost:11434".to_string()),
                embedding_model: Some("nomic-embed-text".to_string()),
            },
        ]
    }

    fn ensure_default_templates(&mut self) {
        let default_templates = Self::default_templates();
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

    pub fn run_smart_setup(&mut self) -> Result<(), String> {
        println!("{}", "🚀 Smart Setup - Logswise CLI".cyan().bold());
        println!("Configure everything you want with intelligent defaults and presets!\n");

        // Check if setup already exists
        let existing_config = self.check_existing_setup();

        if existing_config.is_ok() {
            let action = self.choose_setup_action()?;
            match action {
                SetupAction::UpdateExisting => return self.run_selective_update(),
                SetupAction::CreateNew => {
                    println!("{}", "Creating new configuration...\n".yellow());
                }
                SetupAction::UseTemplate => {
                    return self.setup_from_template();
                }
            }
        }

        let setup_mode = self.choose_setup_mode()?;

        match setup_mode {
            SetupMode::Express => self.run_express_setup(),
            SetupMode::Template => self.setup_from_template(),
            SetupMode::Custom => self.run_custom_setup(),
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

    pub fn run_express_setup(&mut self) -> Result<(), String> {
        println!("{}", "🚀 Express Setup".green().bold());
        println!("Just 5 questions to get you started!\n");

        // 1. Primary role
        let role_options = vec![
            "Software Developer",
            "Data Scientist",
            "DevOps Engineer",
            "Product Manager",
            "Other",
        ];

        let role_idx = Select::new()
            .with_prompt("What's your primary role?")
            .items(&role_options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        // 2. Primary language/tech
        let tech_options = vec![
            "JavaScript/TypeScript",
            "Python",
            "Rust",
            "Go",
            "Java",
            "Other",
        ];

        let tech_idx = Select::new()
            .with_prompt("What's your primary programming language?")
            .items(&tech_options)
            .default(0)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        // 3. Company name (optional)
        let company_name: String = Input::new()
            .with_prompt("Company name (optional, press Enter to skip)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        // 4. LLM preference
        let llm_options = vec![
            "llama3.2 (recommended for general use)",
            "codellama (code-focused)",
            "mistral (lightweight)",
            "custom (I'll specify)",
        ];

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

        // 5. Supabase setup
        let (supabase_url, supabase_api_key) = self.setup_supabase_smart()?;

        // Apply smart defaults based on selections
        let (job_title, company_size, years_experience, work_mode) =
            self.get_smart_defaults(role_options[role_idx]);

        // Save configuration
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

    fn get_smart_defaults(&self, profession: &str) -> (String, String, String, String) {
        match profession {
            "Software Developer" => (
                "Mid".to_string(),
                "10-100".to_string(),
                "3-5 years".to_string(),
                "Remote".to_string(),
            ),
            "Data Scientist" => (
                "Senior".to_string(),
                "100-500".to_string(),
                "3-5 years".to_string(),
                "Hybrid".to_string(),
            ),
            "DevOps Engineer" => (
                "Senior".to_string(),
                "100-500".to_string(),
                "5-10 years".to_string(),
                "Remote".to_string(),
            ),
            "Product Manager" => (
                "Senior".to_string(),
                "100-500".to_string(),
                "5-10 years".to_string(),
                "Hybrid".to_string(),
            ),
            _ => (
                "Mid".to_string(),
                "10-100".to_string(),
                "3-5 years".to_string(),
                "Remote".to_string(),
            ),
        }
    }

    pub fn setup_from_template(&mut self) -> Result<(), String> {
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

        // Allow customization of key fields
        let customize = Confirm::new()
            .with_prompt("Would you like to customize this template?")
            .default(false)
            .interact()
            .map_err(|e| format!("Input error: {e}"))?;

        let mut config_data = self.template_to_config(template);

        if customize {
            config_data = self.customize_template_config(config_data)?;
        }

        // Always need company name and Supabase
        let company_name: String = Input::new()
            .with_prompt("Enter your company name")
            .default("My Company".to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        config_data["companyName"] = json!(company_name);

        let (supabase_url, supabase_api_key) = self.setup_supabase_smart()?;
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

        // Allow changing key fields
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
                    let options = vec![
                        "Intern", "Junior", "Mid", "Senior", "Lead", "Manager", "Director",
                    ];
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "companySize" => {
                    let options = vec![
                        "1-10",
                        "10-100",
                        "100-500",
                        "500-1000",
                        "1000-5000",
                        "5000+",
                    ];
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "yearsExperience" => {
                    let options = vec![
                        "<1 year",
                        "1-3 years",
                        "3-5 years",
                        "5-10 years",
                        "10+ years",
                    ];
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "preferredLanguage" => {
                    let options = vec![
                        "Rust",
                        "Python",
                        "JavaScript/TypeScript",
                        "Go",
                        "Java",
                        "C#",
                        "C/C++",
                        "Other",
                    ];
                    let selection = Select::new()
                        .with_prompt(format!("Select {field_name}"))
                        .items(&options)
                        .interact()
                        .map_err(|e| format!("Input error: {e}"))?;
                    config[field_key] = json!(options[selection]);
                }
                "workMode" => {
                    let options = vec!["Remote", "On-site", "Hybrid"];
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

    fn run_custom_setup(&mut self) -> Result<(), String> {
        println!("{}", "⚙️  Custom Setup".green().bold());
        println!("Full control over your configuration\n");

        // Use the original setup logic but with better UX
        crate::setup::run_setup();
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

        // Validate required fields
        let required_fields = vec!["profession", "supabaseUrl", "supabaseApiKey"];
        for field in required_fields {
            if config.get(field).is_none() {
                return Err(format!("Missing required field: {field}"));
            }
        }

        self.save_configuration(&config)?;

        println!("{}", "✅ Configuration imported successfully!".green());
        Ok(())
    }

    pub fn run_selective_update(&mut self) -> Result<(), String> {
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
                0 => self.update_profile_info(&current_config)?,
                1 => self.update_technical_preferences(&current_config)?,
                2 => self.update_work_environment(&current_config)?,
                3 => self.update_technical_setup(&current_config)?,
                4 => self.update_database_settings(&current_config)?,
                5 => self.save_current_as_template(&current_config)?,
                6 => self.show_current_configuration(&current_config)?,
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

    fn update_profile_info(&self, current_config: &Value) -> Result<(), String> {
        println!("{}", "👤 Updating Profile Information".yellow().bold());

        let mut updated_config = current_config.clone();

        // Update profession
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

        // Update job title
        let job_title_options = vec![
            "Intern", "Junior", "Mid", "Senior", "Lead", "Manager", "Director", "VP", "C-level",
            "Other",
        ];

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

        // Update company name
        let current_company = current_config["companyName"].as_str().unwrap_or("");
        let company_name: String = Input::new()
            .with_prompt(format!("Company Name (current: {current_company})"))
            .default(current_company.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["companyName"] = json!(company_name);

        self.save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_technical_preferences(&self, current_config: &Value) -> Result<(), String> {
        println!("{}", "💻 Updating Technical Preferences".yellow().bold());

        let mut updated_config = current_config.clone();

        // Update preferred language
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

        // Update LLM name
        let current_llm = current_config["llmName"].as_str().unwrap_or("");
        let llm_name: String = Input::new()
            .with_prompt(format!("LLM Name (current: {current_llm})"))
            .default(current_llm.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["llmName"] = json!(llm_name);

        self.save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_work_environment(&self, current_config: &Value) -> Result<(), String> {
        println!("{}", "🏢 Updating Work Environment".yellow().bold());

        let mut updated_config = current_config.clone();

        // Update company size
        let size_options = vec![
            "1-10",
            "10-100",
            "100-500",
            "500-1000",
            "1000-5000",
            "5000+",
        ];
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

        // Update work mode
        let work_mode_options = vec!["Remote", "On-site", "Hybrid"];
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

        self.save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_technical_setup(&self, current_config: &Value) -> Result<(), String> {
        println!("{}", "🔧 Updating Technical Setup".yellow().bold());

        let mut updated_config = current_config.clone();

        // Update Ollama URL
        let current_url = current_config["ollamaBaseUrl"]
            .as_str()
            .unwrap_or("http://localhost:11434");
        let ollama_url: String = Input::new()
            .with_prompt(format!("Ollama Base URL (current: {current_url})"))
            .default(current_url.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["ollamaBaseUrl"] = json!(ollama_url);

        // Update embedding model
        let current_embedding = current_config["embeddingModel"]
            .as_str()
            .unwrap_or("nomic-embed-text");
        let embedding_model: String = Input::new()
            .with_prompt(format!("Embedding Model (current: {current_embedding})"))
            .default(current_embedding.to_string())
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        updated_config["embeddingModel"] = json!(embedding_model);

        self.save_configuration(&updated_config)?;
        Ok(())
    }

    fn update_database_settings(&self, current_config: &Value) -> Result<(), String> {
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

        // Test the connection before saving
        let config = SupabaseConfig {
            project_url: supabase_url.clone(),
            api_key: supabase_api_key.clone(),
        };

        println!("{}", "Testing Supabase connection...".cyan());
        let client = reqwest::blocking::Client::new();

        match test_connection(&client, &config) {
            Ok(_) => {
                println!("{}", "✅ Connection successful!".green());
                updated_config["supabaseUrl"] = json!(supabase_url);
                updated_config["supabaseApiKey"] = json!(supabase_api_key);
                self.save_configuration(&updated_config)?;
            }
            Err(e) => {
                println!("{}", format!("❌ Connection failed: {e}").red());
                println!("{}", "Configuration not updated.".yellow());
            }
        }

        Ok(())
    }

    fn save_current_as_template(&mut self, current_config: &Value) -> Result<(), String> {
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

        self.templates.push(template);
        self.save()?;

        println!("{}", "✅ Template saved successfully!".green());
        Ok(())
    }

    fn show_current_configuration(&self, current_config: &Value) -> Result<(), String> {
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
            ("Supabase URL", "supabaseUrl"),
        ];

        for (label, key) in fields {
            let value = current_config[key].as_str().unwrap_or("Not set");
            println!("{}: {}", label.cyan(), value);
        }

        println!("{}", "─".repeat(50).bright_black());
        Ok(())
    }

    fn setup_supabase_smart(&self) -> Result<(String, String), String> {
        println!("{}", "🗄️  Supabase Configuration".cyan().bold());

        let supabase_url: String = Input::new()
            .with_prompt("Supabase Project URL")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        let supabase_api_key: String = Input::new()
            .with_prompt("Supabase API Key")
            .interact_text()
            .map_err(|e| format!("Input error: {e}"))?;

        // Test connection and setup database
        let config = SupabaseConfig {
            project_url: supabase_url.clone(),
            api_key: supabase_api_key.clone(),
        };

        println!("{}", "Testing connection...".cyan());
        let client = reqwest::blocking::Client::new();

        match test_connection(&client, &config) {
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

        // Check and setup database schema
        match check_notes_table_exists(&client, &config) {
            Ok(false) => {
                let auto_setup = Confirm::new()
                    .with_prompt("Set up database tables automatically?")
                    .default(true)
                    .interact()
                    .map_err(|e| format!("Input error: {e}"))?;

                if auto_setup {
                    match setup_database_schema(&client, &config) {
                        Ok(_) => println!("{}", "✅ Database schema created!".green()),
                        Err(e) => println!("{}", format!("⚠️  Auto-setup failed: {e}").yellow()),
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

#[derive(Debug)]
enum SetupAction {
    UpdateExisting,
    CreateNew,
    UseTemplate,
}

#[derive(Debug)]
enum SetupMode {
    Express,
    Template,
    Custom,
    Import,
}
