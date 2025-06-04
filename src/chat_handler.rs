// use crate::types::SupabaseConfig;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use colored::*;
use reqwest::blocking::Client;
use serde_json::Value;

// fn load_supabase_config() -> SupabaseConfig {
//     let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
//     setup_path.push(".logswise/setup.json");
//     let data = fs::read_to_string(&setup_path)
//         .expect("Setup not found. Please run 'lw setup' first.");
//     let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
//     SupabaseConfig {
//         project_url: profile["supabaseUrl"].as_str().unwrap().to_string(),
//         api_key: profile["supabaseApiKey"].as_str().unwrap().to_string(),
//     }
// }

pub fn chat_with_assistant(message: &str) {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data = fs::read_to_string(&setup_path)
        .expect("Setup not found. Please run 'lw setup' first.");
    let profile: Value = serde_json::from_str(&data).unwrap();
    let llm_name = profile["llmName"].as_str().unwrap_or("llama3");
    let ollama_url = profile.get("ollamaUrl").and_then(|v| v.as_str()).unwrap_or("http://localhost:11434/api/generate");
    let user_info = format!(
        "User Info:\n- Profession: {}\n- Job Title: {}\n- Company Name: {}\n- Company Size: {}",
        profile["profession"].as_str().unwrap_or(""),
        profile["jobTitle"].as_str().unwrap_or(""),
        profile["companyName"].as_str().unwrap_or(""),
        profile["companySize"].as_str().unwrap_or("")
    );
    // Compose the full prompt
    let full_prompt = format!("{}\n\nUser: {}\nAssistant:", user_info, message);
    let client = Client::new();
    let body = serde_json::json!({
        "model": llm_name,
        "prompt": full_prompt,
        "stream": false
    });
    let res = client.post(ollama_url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send();
    match res {
        Ok(resp) if resp.status().is_success() => {
            let data: Value = resp.json().unwrap_or(Value::Null);
            let response = data["response"].as_str().unwrap_or("[Ollama] No response from model.");
            println!("{} {}", "🤖 Assistant:".magenta(), response.white());
        },
        Ok(_) => {
            println!("{}", "[Ollama] No response from model.".red());
        },
        Err(_) => {
            println!("{}", "[Ollama] Error connecting to local Ollama server.".red());
        }
    }
}
