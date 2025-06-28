use crate::types::SupabaseConfig;
use colored::*;
use serde_json::Value;

/// Validates a URL format
#[allow(dead_code)] // Will be used in future improvements
pub fn validate_url(url: &str) -> bool {
    url.trim().starts_with("http://") || url.trim().starts_with("https://")
}

/// Validates an API key (basic checks)
#[allow(dead_code)] // Will be used in future improvements
pub fn validate_api_key(key: &str) -> bool {
    let trimmed = key.trim();
    !trimmed.is_empty() && trimmed.len() >= 10 && !trimmed.contains(' ')
}

/// Validates model name
#[allow(dead_code)] // Will be used in future improvements
pub fn validate_model_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.len() <= 100 && !trimmed.contains('"')
}

/// Validates the entire configuration
#[allow(dead_code)] // Will be used in future improvements
pub fn validate_config(config: &Value) -> Vec<String> {
    let mut errors = Vec::new();

    // Check required fields
    let required_fields = [
        "profession",
        "jobTitle",
        "companyName",
        "companySize",
        "llmName",
        "ollamaBaseUrl",
        "embeddingModel",
        "supabaseUrl",
        "supabaseApiKey",
    ];

    for field in &required_fields {
        if config.get(field).is_none() || config[field].as_str().unwrap_or("").trim().is_empty() {
            errors.push(format!("Missing or empty field: {field}"));
        }
    }

    // Validate URLs
    if let Some(url) = config["supabaseUrl"].as_str() {
        if !validate_url(url) {
            errors.push("Invalid Supabase URL format".to_string());
        }
    }

    if let Some(url) = config["ollamaBaseUrl"].as_str() {
        if !validate_url(url) {
            errors.push("Invalid Ollama base URL format".to_string());
        }
    }

    // Validate API key
    if let Some(key) = config["supabaseApiKey"].as_str() {
        if !validate_api_key(key) {
            errors.push("Invalid Supabase API key format".to_string());
        }
    }

    // Validate model names
    if let Some(model) = config["llmName"].as_str() {
        if !validate_model_name(model) {
            errors.push("Invalid LLM name format".to_string());
        }
    }

    if let Some(model) = config["embeddingModel"].as_str() {
        if !validate_model_name(model) {
            errors.push("Invalid embedding model name format".to_string());
        }
    }

    errors
}

/// Checks if configuration is valid and prints helpful error messages
#[allow(dead_code)] // Will be used in future improvements
pub fn check_config_health(_config: &SupabaseConfig, profile: &Value) -> bool {
    let validation_errors = validate_config(profile);

    if !validation_errors.is_empty() {
        println!("{}", "⚠️  Configuration validation issues found:".yellow());
        for error in validation_errors {
            println!("   • {error}");
        }
        println!(
            "\n{}",
            "Run 'logswise-cli setup' to fix these issues.".cyan()
        );
        return false;
    }

    true
}
