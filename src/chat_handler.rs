// use crate::types::SupabaseConfig;
use colored::*;
use dirs::home_dir;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Chats with the assistant using the configured LLM and user profile.
pub fn chat_with_assistant(message: &str) {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data = fs::read_to_string(&setup_path)
        .expect("Setup not found. Please run 'lw setup' first.");
    let profile: Value = serde_json::from_str(&data).unwrap();
    let llm_name = profile["llmName"].as_str().unwrap_or("").to_lowercase();
    if !llm_name.is_empty() {
        let ollama_url = profile.get("ollamaUrl")
            .and_then(|v| v.as_str())
            .unwrap_or("http://localhost:11434/api/generate");
        // Start spinner
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));
        spinner.set_message("Loading profile and preparing chat...");
        // Gather user info for context
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
            // "stream": false // Uncomment if your Ollama version requires it
        });
        println!("💬 Using Ollama model: {}", llm_name.cyan());
        spinner.set_message("Ollama: Sending request...");
        let res = client
            .post(ollama_url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send();
        match res {
            Ok(resp) => {
                let status = resp.status();
                println!("{} {}", "🔄 Ollama response status:".cyan(), status);
                if status.is_success() {
                    spinner.set_message("Ollama: Success! Generating response...");
                    let raw_body = resp.text().unwrap_or_default();
                    let mut final_response = String::new();
                    for line in raw_body.lines() {
                        if let Ok(data) = serde_json::from_str::<Value>(line) {
                            if let Some(resp_str) = data.get("response").and_then(|v| v.as_str()) {
                                final_response.push_str(resp_str);
                            }
                        }
                    }
                    if final_response.trim().is_empty() {
                        if let Ok(data) = serde_json::from_str::<Value>(&raw_body) {
                            if let Some(resp_str) = data.get("response").and_then(|v| v.as_str()) {
                                final_response.push_str(resp_str);
                            } else {
                                spinner.finish_and_clear();
                                println!(
                                    "⚠️  No 'response' field in Ollama output. Full JSON: {}",
                                    serde_json::to_string_pretty(&data)
                                        .unwrap_or_default()
                                        .yellow()
                                );
                            }
                        } else {
                            spinner.finish_and_clear();
                            println!("⚠️  Ollama output is not valid JSON: {}", raw_body.yellow());
                            if !raw_body.trim().is_empty() {
                                println!(
                                    "{}\n{}",
                                    "🤖 Assistant (raw output):".green(),
                                    raw_body.trim()
                                );
                                return;
                            }
                        }
                    }
                    spinner.finish_and_clear();
                    if !final_response.trim().is_empty() {
                        let final_answer = if let Some(idx) = final_response.find("</think>") {
                            final_response[(idx + "</think>".len())..].trim()
                        } else {
                            final_response.trim()
                        };
                        println!("{}\n{}", "🤖 Assistant:".magenta(), final_answer.white());
                    } else {
                        println!("{} {}", "❌ No response from model:".red(), llm_name);
                        println!("{}", "--- Full Ollama response for debugging ---".yellow());
                        println!("{}", raw_body.cyan());
                    }
                } else {
                    spinner.finish_and_clear();
                    let status = resp.status();
                    let err_body = resp.text().unwrap_or_default();
                    println!("❌ Ollama server returned error status: {}", status);
                    println!("{}", err_body.red());
                    println!(
                        "{}",
                        "➡️  Please check your model name and prompt. See Ollama logs for details."
                            .yellow()
                    );
                }
            }
            Err(e) => {
                spinner.finish_and_clear();
                println!(
                    "{} {}",
                    "❌ Error connecting to local Ollama server for model:".red(),
                    llm_name
                );
                println!("{}", e);
                println!("{}", "➡️  Please ensure the Ollama server is running at http://localhost:11434. You can start it with 'ollama serve' or check your setup.json for the correct URL.".yellow());
            }
        }
    } else {
        println!(
            "{}",
            "😅 No LLM configured. Please set up your LLM in setup.json.".yellow()
        );
    }
}
