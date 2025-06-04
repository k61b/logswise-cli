use crate::types::SupabaseConfig;
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use colored::*;
use reqwest::blocking::Client;
use serde_json::Value;

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

fn load_profile() -> serde_json::Value {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data = fs::read_to_string(&setup_path)
        .expect("Setup not found. Please run 'lw setup' first.");
    serde_json::from_str(&data).unwrap()
}

pub fn get_suggestions(query: &str) {
    let profile = load_profile();
    let llm_name = profile["llmName"].as_str().unwrap_or("").to_lowercase();
    if !llm_name.is_empty() {
        let ollama_url = profile["ollamaUrl"].as_str().unwrap_or("http://localhost:11434/api/generate");
        // Gather user info for context
        let user_info = format!(
            "User Info:\n- Profession: {}\n- Job Title: {}\n- Company Name: {}\n- Company Size: {}",
            profile["profession"].as_str().unwrap_or(""),
            profile["jobTitle"].as_str().unwrap_or(""),
            profile["companyName"].as_str().unwrap_or(""),
            profile["companySize"].as_str().unwrap_or("")
        );
        // Gather last 3 notes for context (if possible)
        let mut notes_context = String::new();
        let config = load_supabase_config();
        let client = Client::new();
        let notes_url = format!("{}/rest/v1/notes?select=content&order=created_at.desc&limit=3", config.project_url);
        let notes_res = client.get(&notes_url)
            .header("apikey", &config.api_key)
            .header("Authorization", format!("Bearer {}", &config.api_key))
            .header("Accept", "application/json")
            .send();
        if let Ok(resp) = notes_res {
            if let Ok(notes_val) = resp.json::<Value>() {
                if let Some(arr) = notes_val.as_array() {
                    if !arr.is_empty() {
                        notes_context.push_str("\nRecent Notes:");
                        for (i, n) in arr.iter().enumerate() {
                            let content = n["content"].as_str().unwrap_or("");
                            notes_context.push_str(&format!("\n{}. {}", i + 1, content));
                        }
                    }
                }
            }
        }
        // Compose the full prompt for Ollama with CLI-optimized instructions
        let mut personalization = String::new();
        if profile["companyName"].as_str().unwrap_or("") != "" && profile["companySize"].as_str().unwrap_or("") != "" {
            personalization.push_str(&format!(
                "At {} scale ({}), consider centralized log filtering and context-rich logs.\n",
                profile["companyName"].as_str().unwrap_or(""),
                profile["companySize"].as_str().unwrap_or("")
            ));
        }
        if let Some(profession) = profile["profession"].as_str() {
            if profession.to_lowercase().contains("developer") {
                personalization.push_str("As a developer, concise logs and clear error levels help with fast debugging.\n");
            }
        }
        personalization.push_str("Since you're using Supabase, log DB connection and query phases for traceability.\n");
        let cli_instruction = "Reply in this format:\n=== Quick Summary ===\n(3-line summary)\n=== Detailed Suggestions ===\n(Max 10 concise bullets, no markdown, CLI readable)";
        let full_prompt = format!(
            "{}{}\n\nUser wants suggestions for: {}\n{}{}\nSuggestions:",
            user_info, notes_context, query, personalization, cli_instruction
        );
        // Send prompt to Ollama
        let ollama_body = serde_json::json!({
            "model": llm_name,
            "prompt": full_prompt,
            // Remove or comment out 'stream' if not needed by your Ollama version
            // "stream": false
        });
        println!("{} {}", "🔎 Using Ollama model:", llm_name.cyan());
        let ollama_res = client.post(ollama_url)
            .header("Content-Type", "application/json")
            .json(&ollama_body)
            .send();
        match ollama_res {
            Ok(resp) => {
                println!("{} {}", "🔄 Ollama response status:".cyan(), resp.status());
                if resp.status().is_success() {
                    let raw_body = resp.text().unwrap_or_default();
                    let mut final_response = String::new();
                    for line in raw_body.lines() {
                        if let Ok(data) = serde_json::from_str::<Value>(line) {
                            if let Some(resp_str) = data.get("response").and_then(|v| v.as_str()) {
                                final_response.push_str(resp_str);
                            }
                        }
                    }
                    if !final_response.trim().is_empty() {
                        // Optionally, only show the part after </think> if present
                        let final_answer = if let Some(idx) = final_response.find("</think>") {
                            final_response[(idx + "</think>".len())..].trim()
                        } else {
                            final_response.trim()
                        };
                        println!("{}\n{}", "💡 Suggestions:".green(), final_answer);
                    } else {
                        println!("{} {}", "❌ No suggestion from model:".red(), llm_name);
                        println!("{}", "--- Full Ollama response for debugging ---".yellow());
                        println!("{}", raw_body.cyan());
                    }
                } else {
                    let status = resp.status();
                    let err_body = resp.text().unwrap_or_default();
                    println!("{} {}", "❌ Ollama server returned error status:", status);
                    println!("{}", err_body.red());
                    println!("{}", "➡️  Please check your model name and prompt. See Ollama logs for details.".yellow());
                }
            },
            Err(e) => {
                println!("{} {}", "❌ Error connecting to local Ollama server for model:".red(), llm_name);
                println!("{}", e);
                println!("{}", "➡️  Please ensure the Ollama server is running at http://localhost:11434. You can start it with 'ollama serve' or check your setup.json for the correct URL.".yellow());
            }
        }
    } else {
        println!("{}", "😅 No LLM configured. Please set up your LLM in setup.json.".yellow());
    }
}
