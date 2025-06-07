use crate::types::SupabaseConfig;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

pub fn load_supabase_config() -> SupabaseConfig {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data =
        fs::read_to_string(&setup_path).expect("Setup not found. Please run 'lw setup' first.");
    let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
    SupabaseConfig {
        project_url: profile["supabaseUrl"].as_str().unwrap().to_string(),
        api_key: profile["supabaseApiKey"].as_str().unwrap().to_string(),
    }
}

pub fn load_profile() -> serde_json::Value {
    let mut setup_path = home_dir().unwrap_or(PathBuf::from("."));
    setup_path.push(".logswise/setup.json");
    let data =
        fs::read_to_string(&setup_path).expect("Setup not found. Please run 'lw setup' first.");
    serde_json::from_str(&data).unwrap()
}
