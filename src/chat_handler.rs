use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::time::Duration;

use crate::services::ollama::generate_suggestion;
use crate::services::supabase::semantic_search_notes;
use crate::utils::{load_profile, load_supabase_config};

/// Chats with the assistant using the configured LLM, user profile, and recent notes.
pub fn chat_with_assistant(message: &str) {
    let profile = match load_profile() {
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
    let ollama_url = format!("{ollama_base_url}/api/generate");
    let ollama_embedding_url = format!("{ollama_base_url}/api/embeddings");
    let ollama_model = profile["embeddingModel"]
        .as_str()
        .unwrap_or("nomic-embed-text");
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
    } else {
        println!(
            "🧠 Running in normal LLM mode (may be slow). Model: {}",
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

    // Load Supabase config
    let config = match load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {e}").red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };
    let client = Client::new();

    // Generate embedding for the chat message
    let query_embedding = match crate::services::ollama::generate_embedding(
        &client,
        &ollama_embedding_url,
        ollama_model,
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

    // Query Supabase for most similar notes (top 5)
    let notes = semantic_search_notes(&client, &config, &query_embedding, 5);
    let mut notes_context = String::new();
    if !notes.is_empty() {
        notes_context.push_str("\nRelevant Notes:");
        for (i, content) in notes.iter().enumerate() {
            notes_context.push_str(&format!("\n{}. {}", i + 1, content));
        }
    }

    // If embedding-only mode, just show notes and return
    if is_embedding {
        spinner.finish_and_clear();
        if !notes.is_empty() {
            println!("\nRelevant Notes:");
            for (i, content) in notes.iter().enumerate() {
                println!("{}. {}", i + 1, content);
            }
        } else {
            println!("No relevant notes found.");
        }
        return;
    }

    // Normal LLM mode: prepare context and generate response
    let user_info = format!(
        "User Info:\n- Profession: {}\n- Job Title: {}\n- Company Name: {}\n- Company Size: {}",
        profile["profession"].as_str().unwrap_or(""),
        profile["jobTitle"].as_str().unwrap_or(""),
        profile["companyName"].as_str().unwrap_or(""),
        profile["companySize"].as_str().unwrap_or("")
    );
    // Compose the full prompt for Ollama
    let full_prompt = format!("{user_info}{notes_context}\n\nUser: {message}\nAssistant:");
    spinner.set_message("Ollama: Sending request...");
    match generate_suggestion(&client, &ollama_url, &llm_name, &full_prompt) {
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
