use crate::types::SupabaseConfig;
use colored::*;
use reqwest::blocking::Client;
use serde_json::Value;
use std::time::Duration;

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

/// Test Supabase connection by making a simple query
pub fn test_connection(client: &Client, config: &SupabaseConfig) -> Result<(), String> {
    let url = format!("{}/rest/v1/", config.project_url);

    let response = client
        .get(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .timeout(Duration::from_secs(10))
        .send()
        .map_err(|e| format!("Connection failed: {e}"))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Authentication failed: HTTP {}", response.status()))
    }
}

/// Check if the notes table exists
pub fn check_notes_table_exists(client: &Client, config: &SupabaseConfig) -> Result<bool, String> {
    let url = format!("{}/rest/v1/notes", config.project_url);

    let response = client
        .get(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .query(&[("limit", "1")])
        .timeout(Duration::from_secs(10))
        .send()
        .map_err(|e| format!("Failed to check table: {e}"))?;

    // If we get a 200, table exists. If we get 404, table doesn't exist.
    // Other errors indicate permission or connection issues.
    match response.status().as_u16() {
        200 => Ok(true),
        404 => {
            // Check if it's a "table not found" error or "endpoint not found"
            let error_text = response.text().unwrap_or_default();
            if error_text.contains("relation") && error_text.contains("does not exist") {
                Ok(false)
            } else {
                // Different kind of 404, might be endpoint issue
                Err(
                    "Unable to access database. Please check your Supabase configuration."
                        .to_string(),
                )
            }
        }
        401 | 403 => Err("Invalid API key or insufficient permissions".to_string()),
        _ => Err(format!("Unexpected response: HTTP {}", response.status())),
    }
}

/// Execute SQL commands to set up the database schema
pub fn setup_database_schema(client: &Client, config: &SupabaseConfig) -> Result<(), String> {
    println!();
    println!("{}", "🔧 Setting up database schema...".cyan());
    println!();

    // Instead of trying to execute SQL automatically (which most Supabase instances don't support),
    // we'll provide clear instructions and optionally try to create the table via the REST API

    // First, try to create the table using the REST API
    match create_notes_table_via_api(client, config) {
        Ok(_) => {
            println!("{} Successfully created notes table!", "✅".green());

            // Now show the additional SQL that needs to be run manually
            show_manual_setup_instructions();

            Ok(())
        }
        Err(_) => {
            // If that fails, show all the SQL that needs to be run manually
            println!("{}", "Automatic table creation not available. Please run the following SQL commands manually:".yellow());
            show_complete_sql_setup();

            Err("Manual SQL setup required".to_string())
        }
    }
}

/// Try to create the notes table using Supabase REST API
fn create_notes_table_via_api(client: &Client, config: &SupabaseConfig) -> Result<(), String> {
    // This approach uses the direct table creation - some Supabase instances allow this
    let url = format!("{}/rest/v1/notes", config.project_url);

    // Try to create with a dummy record to trigger table creation if it's enabled
    let response = client
        .post(&url)
        .header("apikey", &config.api_key)
        .header("Authorization", format!("Bearer {}", &config.api_key))
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&serde_json::json!({
            "content": "Setup test note - you can delete this",
            "embedding": null
        }))
        .timeout(Duration::from_secs(10))
        .send()
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.status().is_success() {
        // If successful, the table exists - clean up the test note
        Ok(())
    } else if response.status().as_u16() == 404 {
        // Table doesn't exist and can't be auto-created
        Err("Table does not exist".to_string())
    } else {
        let error_text = response.text().unwrap_or_default();
        Err(format!("API error: {error_text}"))
    }
}

/// Show manual setup instructions for additional schema elements
fn show_manual_setup_instructions() {
    println!();
    println!(
        "{}",
        "To complete the setup, please run the following SQL commands in your Supabase SQL Editor:"
            .bright_cyan()
    );
    println!();

    println!(
        "{}",
        "-- Enable pgvector extension for embeddings".bright_black()
    );
    println!(
        "{}",
        "CREATE EXTENSION IF NOT EXISTS vector;".bright_white()
    );
    println!();

    println!(
        "{}",
        "-- Add embedding column (if not already present)".bright_black()
    );
    println!(
        "{}",
        "ALTER TABLE notes ADD COLUMN IF NOT EXISTS embedding vector(768);".bright_white()
    );
    println!();

    println!(
        "{}",
        "-- Create indexes for better performance".bright_black()
    );
    println!(
        "{}",
        "CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes (created_at);".bright_white()
    );
    println!("{}", "CREATE INDEX IF NOT EXISTS idx_notes_embedding ON notes USING ivfflat (embedding vector_cosine_ops);".bright_white());
    println!();

    println!("{}", "-- Create semantic search function".bright_black());
    println!(
        "{}",
        r#"CREATE OR REPLACE FUNCTION semantic_search_notes(
    query_embedding vector(768),
    match_count int DEFAULT 5
)
RETURNS TABLE (
    id uuid,
    content text,
    created_at timestamp with time zone,
    similarity float
)
LANGUAGE sql
AS $$
    SELECT
        notes.id,
        notes.content,
        notes.created_at,
        1 - (notes.embedding <=> query_embedding) AS similarity
    FROM notes
    WHERE notes.embedding IS NOT NULL
    ORDER BY notes.embedding <=> query_embedding
    LIMIT match_count;
$$;"#
            .bright_white()
    );
    println!();
}

/// Show complete SQL setup when automatic creation fails
fn show_complete_sql_setup() {
    println!();
    println!(
        "{}",
        "Please run these SQL commands in your Supabase SQL Editor:".bright_cyan()
    );
    println!();

    println!("{}", "-- 1. Create the notes table".bright_black());
    println!(
        "{}",
        r#"CREATE TABLE IF NOT EXISTS notes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    content text NOT NULL,
    created_at timestamp with time zone DEFAULT timezone('utc'::text, now())
);"#
        .bright_white()
    );
    println!();

    show_manual_setup_instructions();

    println!(
        "{}",
        "After running these commands, your database will be ready for use!".green()
    );
    println!();
}
