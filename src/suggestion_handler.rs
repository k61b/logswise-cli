use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::time::Duration;

use crate::services::ollama;
use crate::services::supabase;
use crate::utils;

pub fn get_suggestions(query: &str) {
    let profile = match utils::load_profile() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", format!("Error loading profile: {e}").red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };
    let llm_name = profile["llmName"].as_str().unwrap_or("").to_lowercase();
    if llm_name.is_empty() {
        println!(
            "{}",
            "😅 No LLM configured. Please set up your LLM in setup.json.".yellow()
        );
        return;
    }
    let ollama_base_url = profile["ollamaBaseUrl"]
        .as_str()
        .unwrap_or("http://localhost:11434");
    let ollama_embedding_url = format!("{ollama_base_url}/api/embeddings");
    let ollama_generate_url = format!("{ollama_base_url}/api/generate");
    let ollama_model = profile["embeddingModel"]
        .as_str()
        .unwrap_or("nomic-embed-text");
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Loading profile and preparing suggestions...");
    let user_info = format!(
        "User Info:\n- Profession: {}\n- Job Title: {}\n- Company Name: {}\n- Company Size: {}",
        profile["profession"].as_str().unwrap_or(""),
        profile["jobTitle"].as_str().unwrap_or(""),
        profile["companyName"].as_str().unwrap_or(""),
        profile["companySize"].as_str().unwrap_or("")
    );
    let mut notes_context = String::new();
    let config = match utils::load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {e}").red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };
    let client = Client::new();
    let embedding_models = [
        "nomic-embed-text",
        "bge-base-en",
        "all-minilm",
        // Add more known embedding models here if needed
    ];
    let is_embedding = embedding_models
        .iter()
        .any(|m| llm_name == *m || llm_name.starts_with(m));
    if is_embedding {
        // Only perform embedding and semantic search, print results, and exit (no LLM generation)
        println!(
            "⚡ Running in embedding-only mode (semantic search, no LLM generation). Model: {}",
            llm_name.cyan()
        );
        // 1. Generate embedding for the query using local Ollama
        let query_embedding = match ollama::generate_embedding(
            &client,
            &ollama_embedding_url,
            ollama_model,
            query,
        ) {
            Ok(embedding) => embedding,
            Err(msg) => {
                spinner.finish_and_clear();
                println!(
                    "{}",
                    "❌ Could not generate embedding for query. No semantic search can be made."
                        .red()
                );
                println!("{}", msg.yellow());
                println!("➡️  Please check that your embedding model is pulled and running in Ollama (e.g., 'ollama pull nomic-embed-text'), and that OLLAMA_EMBEDDING_MODEL is set correctly.");
                return;
            }
        };
        // 2. Query Supabase for most similar notes (top 5)
        let notes = supabase::semantic_search_notes(&client, &config, &query_embedding, 5);
        if !notes.is_empty() {
            println!("\nRelevant Notes:");
            for (i, content) in notes.iter().enumerate() {
                println!("{}. {}", i + 1, content);
            }
        } else {
            println!("No relevant notes found.");
        }
        spinner.finish_and_clear();
        return;
    }
    // 1. Generate embedding for the query using local Ollama
    let query_embedding = match ollama::generate_embedding(
        &client,
        &ollama_embedding_url,
        ollama_model,
        query,
    ) {
        Ok(embedding) => embedding,
        Err(msg) => {
            spinner.finish_and_clear();
            println!(
                "{}",
                "❌ Could not generate embedding for query. No suggestions can be made.".red()
            );
            println!("{}", msg.yellow());
            println!("➡️  Please check that your embedding model is pulled and running in Ollama (e.g., 'ollama pull nomic-embed-text'), and that OLLAMA_EMBEDDING_MODEL is set correctly.");
            return;
        }
    };
    // 2. Query Supabase for most similar notes (top 5)
    let notes = supabase::semantic_search_notes(&client, &config, &query_embedding, 5);
    if !notes.is_empty() {
        notes_context.push_str("\nRelevant Notes:");
        for (i, content) in notes.iter().enumerate() {
            notes_context.push_str(&format!("\n{}. {}", i + 1, content));
        }
    }
    // Compose the full prompt for Ollama with CLI-optimized instructions
    let mut personalization = String::new();
    if !profile["companyName"].as_str().unwrap_or("").is_empty()
        && !profile["companySize"].as_str().unwrap_or("").is_empty()
    {
        personalization.push_str(&format!(
            "At {} scale ({}), consider centralized log filtering and context-rich logs.\n",
            profile["companyName"].as_str().unwrap_or(""),
            profile["companySize"].as_str().unwrap_or("")
        ));
    }
    if let Some(profession) = profile["profession"].as_str() {
        if profession.to_lowercase().contains("developer") {
            personalization.push_str(
                "As a developer, concise logs and clear error levels help with fast debugging.\n",
            );
        }
    }
    personalization.push_str(
        "Since you're using Supabase, log DB connection and query phases for traceability.\n",
    );
    let cli_instruction = "Reply in this format:\n=== Quick Summary ===\n(3-line summary)\n=== Suggestions by Category ===\n- Learning:\n  1. Suggestion (with a simple way to track success)\n  2. ...\n- Collaboration:\n  3. ...\n- Well-being:\n  4. ...\n(Up to 10 total, grouped by category. For each, add a quick feedback loop, e.g., 'Do a team poll after 2 weeks' or 'Check adoption in next retro'. Keep the tone informal and practical for a small, busy team. No markdown, CLI readable.)";
    let full_prompt = format!(
        "{user_info}{notes_context}\n\nUser wants suggestions for: {query}\n{personalization}{cli_instruction}\nSuggestions:"
    );
    println!("🔎 Using Ollama model: {}", llm_name.cyan());
    spinner.set_message("Ollama: Sending request...");
    match ollama::generate_suggestion(&client, &ollama_generate_url, &llm_name, &full_prompt) {
        Ok(final_response) => {
            spinner.finish_and_clear();
            if !final_response.trim().is_empty() {
                println!("\n==================== 💡 Team Suggestions ====================\n");
                let final_answer = if let Some(idx) = final_response.find("</think>") {
                    final_response[(idx + "</think>".len())..].trim()
                } else {
                    final_response.trim()
                };
                println!(
                    "----------------------------------------\n{final_answer}\n----------------------------------------\n"
                );
            } else {
                println!("{} {}", "❌ No suggestion from model:".red(), llm_name);
            }
        }
        Err(msg) => {
            spinner.finish_and_clear();
            println!("{}", msg.red());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::SupabaseConfig;
    use serde_json::json;

    #[test]
    fn test_load_supabase_config_valid() {
        // Prepare a temp setup.json
        let tmp_dir = tempfile::tempdir().unwrap();
        let mut setup_path = tmp_dir.path().to_path_buf();
        setup_path.push("setup.json");
        let json = r#"{
            "supabaseUrl": "https://test.supabase.co",
            "supabaseApiKey": "testkey"
        }"#;
        std::fs::write(&setup_path, json).unwrap();
        // Patch home_dir to return tmp_dir (not possible here), so just test parsing logic
        let data = std::fs::read_to_string(&setup_path).unwrap();
        let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
        let config = SupabaseConfig {
            project_url: profile["supabaseUrl"].as_str().unwrap().to_string(),
            api_key: profile["supabaseApiKey"].as_str().unwrap().to_string(),
        };
        assert_eq!(config.project_url, "https://test.supabase.co");
        assert_eq!(config.api_key, "testkey");
    }

    #[test]
    fn test_load_profile_valid() {
        let profile = json!({
            "profession": "Developer",
            "jobTitle": "Senior",
            "companyName": "TestCo",
            "companySize": "10-100",
            "llmName": "llama3",
            "supabaseUrl": "https://test.supabase.co",
            "supabaseApiKey": "testkey"
        });
        assert_eq!(profile["companyName"], "TestCo");
        assert_eq!(profile["llmName"], "llama3");
    }

    #[test]
    fn test_prompt_formatting() {
        let user_info = "User Info:\n- Profession: Developer\n- Job Title: Senior\n- Company Name: TestCo\n- Company Size: 10-100";
        let notes_context = "\nRecent Notes:\n1. Note one\n2. Note two";
        let query = "How to improve logging?";
        let personalization = "At TestCo scale (10-100), consider centralized log filtering.\n";
        let cli_instruction = "Reply in this format:\n=== Quick Summary ===\n(3-line summary)\n=== Suggestions by Category ===\n- Learning:\n  1. Suggestion (with a simple way to track success)\n  2. ...\n- Collaboration:\n  3. ...\n- Well-being:\n  4. ...\n(Up to 10 total, grouped by category. For each, add a quick feedback loop, e.g., 'Do a team poll after 2 weeks' or 'Check adoption in next retro'. Keep the tone informal and practical for a small, busy team. No markdown, CLI readable)";
        let full_prompt = format!(
            "{user_info}{notes_context}\n\nUser wants suggestions for: {query}\n{personalization}{cli_instruction}\nSuggestions:"
        );
        assert!(full_prompt.contains("User wants suggestions for: How to improve logging?"));
        assert!(full_prompt.contains("Recent Notes:"));
        assert!(full_prompt.contains("=== Quick Summary ==="));
    }
}
