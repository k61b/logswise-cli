use std::fs;
use std::path::PathBuf;
use dialoguer::{Input, Select};
use colored::*;
use dirs::home_dir;
use serde_json::json;

/// Runs the interactive setup process for user profile and configuration.
pub fn run_setup() {
    let profession_options = vec!["Software Developer", "Product Manager", "Designer"];
    let job_title_options = vec!["Mid", "Senior", "Lead", "Manager"];
    let company_size_options = vec!["1-10", "10-100", "100-500", "500-1000", "1000+"];

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
    let llm_name: String = Input::new()
        .with_prompt("Enter the LLM name (e.g., ollama, llama.cpp):")
        .interact_text()
        .unwrap();
    let supabase_url: String = Input::new()
        .with_prompt("Enter your Supabase project URL:")
        .interact_text()
        .unwrap();
    let supabase_api_key: String = Input::new()
        .with_prompt("Enter your Supabase API key:")
        .interact_text()
        .unwrap();

    let profile_data = json!({
        "profession": profession_options[profession],
        "jobTitle": job_title_options[job_title],
        "companyName": company_name,
        "companySize": company_size_options[company_size],
        "llmName": llm_name,
        "supabaseUrl": supabase_url,
        "supabaseApiKey": supabase_api_key
    });

    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise");
    fs::create_dir_all(&setup_path).unwrap();
    setup_path.push("setup.json");
    fs::write(&setup_path, serde_json::to_string_pretty(&profile_data).unwrap()).unwrap();
    println!("{}", "✅ Setup complete! You are ready to use Logswise CLI!".green());
}
