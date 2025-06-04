use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: String, // UUID
    pub content: String,
    pub created_at: String, // timestamp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub profession: String,
    pub job_title: String,
    pub company_name: String,
    pub company_size: String,
    pub llm_name: String,
    pub supabase_url: String,
    pub supabase_api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupabaseConfig {
    pub project_url: String,
    pub api_key: String,
}
