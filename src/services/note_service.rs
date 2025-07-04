//! Note service for managing notes with proper separation of concerns.

use crate::errors::{AppError, AppResult};
use crate::services::config::{load_profile_typed, load_supabase_config_typed};
use crate::services::ollama;
use crate::types::{Note, SupabaseConfig};
use reqwest::Client;
use serde_json::json;

pub struct NoteService {
    #[allow(dead_code)]
    client: Client,
}

impl NoteService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Add a note with optional embedding generation
    #[allow(dead_code)]
    pub async fn add_note(&self, content: &str) -> AppResult<()> {
        self.validate_content(content)?;

        let config = load_supabase_config_typed()?;
        let profile = load_profile_typed()?;

        let embedding = self.generate_embedding(&profile, content).await?;
        self.store_note(&config, content, embedding).await?;

        Ok(())
    }

    /// Retrieve recent notes
    #[allow(dead_code)]
    pub async fn get_recent_notes(&self, count: usize) -> AppResult<Vec<Note>> {
        let config = load_supabase_config_typed()?;
        let url = format!("{}/rest/v1/notes", config.project_url);

        let response = self
            .client
            .get(&url)
            .header("apikey", &config.api_key)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .query(&[
                ("select", "id,content,created_at,embedding"),
                ("order", "created_at.desc"),
                ("limit", &count.to_string()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Database(format!("HTTP {status}: {error_text}")));
        }

        let notes_json: Vec<serde_json::Value> = response.json().await?;
        let notes = notes_json
            .into_iter()
            .map(|n| Note {
                id: n["id"].as_str().unwrap_or_default().to_string(),
                content: n["content"].as_str().unwrap_or_default().to_string(),
                created_at: n["created_at"].as_str().unwrap_or_default().to_string(),
                embedding: n["embedding"].as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_f64().map(|f| f as f32))
                        .collect()
                }),
            })
            .collect();

        Ok(notes)
    }

    #[allow(dead_code)]
    fn validate_content(&self, content: &str) -> AppResult<()> {
        if content.trim().is_empty() {
            return Err(AppError::InvalidInput(
                "Note content cannot be empty".to_string(),
            ));
        }

        if content.len() > 10000 {
            return Err(AppError::InvalidInput(
                "Note content too long (max 10,000 characters)".to_string(),
            ));
        }

        Ok(())
    }

    #[allow(dead_code)]
    async fn generate_embedding(
        &self,
        profile: &serde_json::Value,
        content: &str,
    ) -> AppResult<Option<Vec<f32>>> {
        let ollama_base_url = profile["ollamaBaseUrl"]
            .as_str()
            .unwrap_or("http://localhost:11434");
        let ollama_url = format!("{ollama_base_url}/api/embeddings");
        let ollama_model = profile["embeddingModel"]
            .as_str()
            .unwrap_or("nomic-embed-text");

        match ollama::generate_embedding_async(&ollama_url, ollama_model, content).await {
            Ok(embedding) => Ok(Some(embedding)),
            Err(_) => Ok(None), // Continue without embedding
        }
    }

    #[allow(dead_code)]
    async fn store_note(
        &self,
        config: &SupabaseConfig,
        content: &str,
        embedding: Option<Vec<f32>>,
    ) -> AppResult<()> {
        let url = format!("{}/rest/v1/notes", config.project_url);
        let body = if let Some(embedding) = embedding {
            json!({ "content": content, "embedding": embedding })
        } else {
            json!({ "content": content })
        };

        let response = self
            .client
            .post(&url)
            .header("apikey", &config.api_key)
            .header("Authorization", format!("Bearer {}", &config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Database(format!("HTTP {status}: {error_text}")));
        }

        Ok(())
    }
}

impl Default for NoteService {
    fn default() -> Self {
        Self::new()
    }
}
