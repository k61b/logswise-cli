use crate::types::SupabaseConfig;
use reqwest::blocking::Client;
use serde_json::Value;

pub fn semantic_search_notes(
    client: &Client,
    config: &SupabaseConfig,
    embedding: &[f32],
    match_count: usize,
) -> Vec<String> {
    let embedding_str = format!(
        "[{}]",
        embedding
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let sql_url = format!("{}/rest/v1/rpc/semantic_search_notes", config.project_url);
    let sql_body =
        serde_json::json!({ "query_embedding": embedding_str, "match_count": match_count });
    let notes_res = client
        .post(&sql_url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .header("Content-Type", "application/json")
        .json(&sql_body)
        .send();
    let mut notes = Vec::new();
    if let Ok(resp) = notes_res {
        if let Ok(notes_val) = resp.json::<Value>() {
            if let Some(arr) = notes_val.as_array() {
                for n in arr {
                    if let Some(content) = n["content"].as_str() {
                        notes.push(content.to_string());
                    }
                }
            }
        }
    }
    notes
}
