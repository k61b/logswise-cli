//! Configuration service for loading and managing application settings.

use crate::errors::{AppError, AppResult};
use crate::types::SupabaseConfig;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

/// Get the path to the setup configuration file.
fn get_setup_path() -> AppResult<PathBuf> {
    let mut path = home_dir()
        .ok_or_else(|| AppError::Config("Could not determine home directory".to_string()))?;
    path.push(".logswise/setup.json");
    Ok(path)
}

/// Load the raw profile data from setup.json.
fn load_raw_profile() -> AppResult<serde_json::Value> {
    let setup_path = get_setup_path()?;
    let data = fs::read_to_string(&setup_path).map_err(|_| {
        AppError::Config("Setup not found. Please run 'logswise-cli setup' first.".to_string())
    })?;
    serde_json::from_str(&data)
        .map_err(|e| AppError::Config(format!("Failed to parse setup.json: {}", e)))
}

/// Load the user profile configuration.
pub fn load_profile() -> Result<serde_json::Value, String> {
    load_raw_profile().map_err(|e| e.to_string())
}

/// Load the user profile configuration (with AppError).
#[allow(dead_code)]
pub fn load_profile_typed() -> AppResult<serde_json::Value> {
    load_raw_profile()
}

/// Load Supabase configuration from the profile.
pub fn load_supabase_config() -> Result<SupabaseConfig, String> {
    load_supabase_config_typed().map_err(|e| e.to_string())
}

/// Load Supabase configuration from the profile (with AppError).
pub fn load_supabase_config_typed() -> AppResult<SupabaseConfig> {
    let profile = load_raw_profile()?;

    let project_url = profile["supabaseUrl"]
        .as_str()
        .ok_or_else(|| AppError::Config("Missing 'supabaseUrl' in setup.json".to_string()))?
        .to_string();

    let api_key = profile["supabaseApiKey"]
        .as_str()
        .ok_or_else(|| AppError::Config("Missing 'supabaseApiKey' in setup.json".to_string()))?
        .to_string();

    Ok(SupabaseConfig {
        project_url,
        api_key,
    })
}

/// Extract Ollama configuration from profile.
pub fn get_ollama_config(profile: &serde_json::Value) -> (String, String) {
    let base_url = profile
        .get("ollamaBaseUrl")
        .and_then(|v| v.as_str())
        .unwrap_or("http://localhost:11434");
    let model = profile
        .get("llmName")
        .and_then(|v| v.as_str())
        .unwrap_or("llama3");

    (format!("{}/api/generate", base_url), model.to_string())
}

/// Extract Ollama configuration from profile (with AppError).
#[allow(dead_code)]
pub fn get_ollama_config_typed(profile: &serde_json::Value) -> AppResult<(String, String)> {
    let base_url = profile
        .get("ollamaBaseUrl")
        .and_then(|v| v.as_str())
        .unwrap_or("http://localhost:11434");

    let model = profile
        .get("llmName")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Config("LLM model name not found in config".to_string()))?;

    Ok((format!("{}/api/generate", base_url), model.to_string()))
}
