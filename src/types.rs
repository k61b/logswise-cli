use serde::{Deserialize, Serialize};

/// Represents a note stored in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    /// Unique identifier (UUID) for the note.
    pub id: String,
    /// The content of the note.
    pub content: String,
    /// Timestamp when the note was created.
    pub created_at: String,
}

/// User profile and configuration for LLM and Supabase.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    /// User's profession (e.g., Developer, Manager).
    pub profession: String,
    /// User's job title.
    pub job_title: String,
    /// Name of the user's company.
    pub company_name: String,
    /// Size of the user's company.
    pub company_size: String,
    /// Name of the LLM model to use.
    pub llm_name: String,
    /// Supabase project URL.
    pub supabase_url: String,
    /// Supabase API key.
    pub supabase_api_key: String,
}

/// Configuration for connecting to Supabase.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupabaseConfig {
    /// Supabase project URL.
    pub project_url: String,
    /// Supabase API key.
    pub api_key: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_note_struct() {
        let note = Note {
            id: "1234".to_string(),
            content: "Test note".to_string(),
            created_at: "2025-06-05T12:00:00Z".to_string(),
        };
        assert_eq!(note.content, "Test note");
    }

    #[test]
    fn test_supabase_config_struct() {
        let config = SupabaseConfig {
            project_url: "https://test.supabase.co".to_string(),
            api_key: "testkey".to_string(),
        };
        assert_eq!(config.api_key, "testkey");
    }

    #[test]
    fn test_load_profile_and_supabase_config() {
        // Create a temp setup.json
        let tmp_dir = tempfile::tempdir().unwrap();
        let mut setup_path = PathBuf::from(tmp_dir.path());
        setup_path.push("setup.json");
        let json = r#"{
            "profession": "Developer",
            "jobTitle": "Senior",
            "companyName": "TestCo",
            "companySize": "10-100",
            "llmName": "llama3",
            "supabaseUrl": "https://test.supabase.co",
            "supabaseApiKey": "testkey"
        }"#;
        fs::write(&setup_path, json).unwrap();
        let data = fs::read_to_string(&setup_path).unwrap();
        let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
        assert_eq!(profile["companyName"], "TestCo");
        assert_eq!(profile["supabaseUrl"], "https://test.supabase.co");
    }
}
