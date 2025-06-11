use crate::types::SupabaseConfig;
use dirs::home_dir;
use std::fs;

pub fn load_supabase_config() -> Result<SupabaseConfig, String> {
    let mut setup_path =
        home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;
    setup_path.push(".logswise/setup.json");
    let data = fs::read_to_string(&setup_path)
        .map_err(|_| "Setup not found. Please run 'logswise-cli setup' first.".to_string())?;
    let profile: serde_json::Value = serde_json::from_str(&data)
        .map_err(|_| "Failed to parse setup.json. Please check the file format.".to_string())?;
    let project_url = profile["supabaseUrl"]
        .as_str()
        .ok_or_else(|| "Missing 'supabaseUrl' in setup.json".to_string())?
        .to_string();
    let api_key = profile["supabaseApiKey"]
        .as_str()
        .ok_or_else(|| "Missing 'supabaseApiKey' in setup.json".to_string())?
        .to_string();
    Ok(SupabaseConfig {
        project_url,
        api_key,
    })
}

pub fn load_profile() -> Result<serde_json::Value, String> {
    let mut setup_path =
        home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;
    setup_path.push(".logswise/setup.json");
    let data = fs::read_to_string(&setup_path)
        .map_err(|_| "Setup not found. Please run 'logswise-cli setup' first.".to_string())?;
    serde_json::from_str(&data)
        .map_err(|_| "Failed to parse setup.json. Please check the file format.".to_string())
}
