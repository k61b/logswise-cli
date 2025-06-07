use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::time::Duration;

use crate::services::ollama::generate_suggestion;
use crate::services::supabase::semantic_search_notes;
use crate::utils::{load_profile, load_supabase_config};

/// Chats with the assistant using the configured LLM, user profile, and recent notes.
pub fn chat_with_assistant(message: &str) {
    let profile = load_profile();
    let llm_name = profile["llmName"].as_str().unwrap_or("").to_lowercase();
    if llm_name.is_empty() {
        println!(
            "{}",
            "😅 No LLM configured. Please set up your LLM in setup.json.".yellow()
        );
        return;
    }
    let ollama_url = profile
        .get("ollamaUrl")
        .and_then(|v| v.as_str())
        .unwrap_or("http://localhost:11434/api/generate");
    let ollama_embedding_url = std::env::var("OLLAMA_EMBEDDING_URL")
        .unwrap_or_else(|_| "http://localhost:11434/api/embeddings".to_string());
    let ollama_model =
        std::env::var("OLLAMA_EMBEDDING_MODEL").unwrap_or_else(|_| "nomic-embed-text".to_string());
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
            "{} {}",
            "⚡ Running in embedding-only mode (semantic search, no LLM generation). Model:",
            llm_name.cyan()
        );
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));
        spinner.set_message("Loading profile and preparing chat context...");
        // Only use user_info if not in embedding-only mode
        // (remove unused variable warning)
        // let user_info = ... (remove this line entirely)
        let config = load_supabase_config();
        let client = Client::new();
        let query_embedding = match crate::services::ollama::generate_embedding(
            &client,
            &ollama_embedding_url,
            &ollama_model,
            message,
        ) {
            Ok(embedding) => embedding,
            Err(msg) => {
                spinner.finish_and_clear();
                println!(
                    "{}",
                    "❌ Could not generate embedding for chat message. No semantic search can be made.".red()
                );
                println!("{}", msg.yellow());
                println!("➡️  Please check that your embedding model is pulled and running in Ollama (e.g., 'ollama pull nomic-embed-text'), and that OLLAMA_EMBEDDING_MODEL is set correctly.");
                return;
            }
        };
        let notes = semantic_search_notes(&client, &config, &query_embedding, 5);
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
    } else {
        println!(
            "{} {}",
            "🧠 Running in normal LLM mode (may be slow). Model:",
            llm_name.cyan()
        );
    }
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Loading profile and preparing chat context...");
    // Only define and use user_info in normal LLM mode
    let mut notes_context = String::new();
    if is_embedding {
        // ...embedding-only mode logic...
        // ...existing code...
        return;
    }
    // Normal LLM mode: define and use user_info and notes_context
    let user_info = format!(
        "User Info:\n- Profession: {}\n- Job Title: {}\n- Company Name: {}\n- Company Size: {}",
        profile["profession"].as_str().unwrap_or(""),
        profile["jobTitle"].as_str().unwrap_or(""),
        profile["companyName"].as_str().unwrap_or(""),
        profile["companySize"].as_str().unwrap_or("")
    );
    let config = load_supabase_config();
    let client = Client::new();
    // 1. Generate embedding for the chat message
    let query_embedding = match crate::services::ollama::generate_embedding(
        &client,
        &ollama_embedding_url,
        &ollama_model,
        message,
    ) {
        Ok(embedding) => embedding,
        Err(msg) => {
            spinner.finish_and_clear();
            println!(
                "{}",
                "❌ Could not generate embedding for chat message.".red()
            );
            println!("{}", msg.yellow());
            return;
        }
    };
    // 2. Query Supabase for most similar notes (top 5)
    let notes = semantic_search_notes(&client, &config, &query_embedding, 5);
    if !notes.is_empty() {
        notes_context.push_str("\nRelevant Notes:");
        for (i, content) in notes.iter().enumerate() {
            notes_context.push_str(&format!("\n{}. {}", i + 1, content));
        }
    }
    // Compose the full prompt for Ollama
    let full_prompt = format!(
        "{}{}\n\nUser: {}\nAssistant:",
        user_info, notes_context, message
    );
    spinner.set_message("Ollama: Sending request...");
    match generate_suggestion(&client, ollama_url, &llm_name, &full_prompt) {
        Ok(response) => {
            spinner.finish_and_clear();
            println!("{}", response.cyan());
        }
        Err(msg) => {
            spinner.finish_and_clear();
            println!("{}", msg.red());
        }
    }
}
