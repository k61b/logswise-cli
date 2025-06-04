use crate::types::{SupabaseConfig};
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use colored::*;
use reqwest::blocking::Client;
use serde_json::json;

fn load_supabase_config() -> SupabaseConfig {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data = fs::read_to_string(&setup_path)
        .expect("Setup not found. Please run 'lw setup' first.");
    let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
    SupabaseConfig {
        project_url: profile["supabaseUrl"].as_str().unwrap().to_string(),
        api_key: profile["supabaseApiKey"].as_str().unwrap().to_string(),
    }
}

pub fn add_note(content: &str) {
    let config = load_supabase_config();
    let client = Client::new();
    let url = format!("{}/rest/v1/notes", config.project_url);
    let body = json!({ "content": content });
    let res = client.post(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send();
    match res {
        Ok(resp) if resp.status().is_success() => {
            println!("{}", "📝 Note added!".green());
        },
        Ok(resp) => {
            println!("{} {}", "❌ Failed to add note:".red(), resp.text().unwrap_or_default());
        },
        Err(e) => {
            println!("{} {}", "❌ Failed to add note:".red(), e);
        }
    }
}
