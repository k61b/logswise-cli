use colored::*;
use dialoguer::{Input, Select};
use dirs::home_dir;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

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
        "yearsExperience": experience_options[years_experience],
        "preferredLanguage": language_options[preferred_language],
        "workMode": work_mode_options[work_mode],
        "llmName": llm_name,
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
