use reqwest::blocking::Client;
use serde_json::Value;
use std::time::Duration;

/// Default timeout for Ollama requests (30 seconds)
const OLLAMA_TIMEOUT: Duration = Duration::from_secs(30);

pub fn generate_embedding(
    client: &Client,
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<Vec<f32>, String> {
    if prompt.trim().is_empty() {
        return Err("Prompt cannot be empty".to_string());
    }

    if model.trim().is_empty() {
        return Err("Model name cannot be empty".to_string());
    }

    let embedding_body = serde_json::json!({
        "model": model,
        "prompt": prompt
    });

    let embedding_res = client
        .post(ollama_url)
        .timeout(OLLAMA_TIMEOUT)
        .header("Content-Type", "application/json")
        .json(&embedding_body)
        .send();
    match embedding_res {
        Ok(resp) if resp.status().is_success() => {
            let resp_text = resp.text().unwrap_or_default();
            if resp_text.is_empty() {
                return Err("Received empty response from Ollama".to_string());
            }

            let resp_json: Value = match serde_json::from_str(&resp_text) {
                Ok(json) => json,
                Err(e) => return Err(format!("Failed to parse Ollama response: {}", e)),
            };

            if resp_json.get("embedding").is_some()
                && resp_json.as_object().map_or(0, |o| o.len()) == 1
            {
                Ok(resp_json["embedding"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64().map(|f| f as f32))
                            .collect()
                    })
                    .unwrap_or_default())
            } else {
                Err(format!(
                    "Unexpected embedding response format: {}",
                    resp_text
                ))
            }
        }
        Ok(resp) => {
            let status = resp.status();
            let err_text = resp.text().unwrap_or_default();
            if status.as_u16() == 404 {
                Err(format!(
                    "Model '{}' not found. Try: ollama pull {}",
                    model, model
                ))
            } else {
                Err(format!(
                    "Ollama returned error status {}: {}",
                    status, err_text
                ))
            }
        }
        Err(e) => {
            if e.is_timeout() {
                Err(
                    "Request to Ollama timed out. Check if Ollama is running and responsive."
                        .to_string(),
                )
            } else if e.is_connect() {
                Err(
                    "Failed to connect to Ollama. Make sure it's running on the correct URL."
                        .to_string(),
                )
            } else {
                Err(format!("Failed to connect to Ollama: {}", e))
            }
        }
    }
}

pub fn generate_suggestion(
    client: &Client,
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    if prompt.trim().is_empty() {
        return Err("Prompt cannot be empty".to_string());
    }

    if model.trim().is_empty() {
        return Err("Model name cannot be empty".to_string());
    }

    let ollama_body = serde_json::json!({
        "model": model,
        "prompt": prompt
    });
    let ollama_res = client
        .post(ollama_url)
        .timeout(OLLAMA_TIMEOUT)
        .header("Content-Type", "application/json")
        .json(&ollama_body)
        .send();
    match ollama_res {
        Ok(resp) if resp.status().is_success() => {
            let raw_body = resp.text().unwrap_or_default();
            if raw_body.is_empty() {
                return Err("Received empty response from Ollama".to_string());
            }

            let mut final_response = String::new();
            for line in raw_body.lines() {
                if let Ok(data) = serde_json::from_str::<Value>(line) {
                    if let Some(resp_str) = data.get("response").and_then(|v| v.as_str()) {
                        final_response.push_str(resp_str);
                    }
                    // Check for errors in streaming response
                    if let Some(error) = data.get("error").and_then(|v| v.as_str()) {
                        return Err(format!("Ollama error: {}", error));
                    }
                }
            }

            if final_response.trim().is_empty() {
                Err("Model generated empty response. Try a different prompt or model.".to_string())
            } else {
                Ok(final_response)
            }
        }
        Ok(resp) => {
            let status = resp.status();
            let err_body = resp.text().unwrap_or_default();
            if status.as_u16() == 404 {
                Err(format!(
                    "Model '{}' not found. Try: ollama pull {}",
                    model, model
                ))
            } else {
                Err(format!(
                    "Ollama server returned error status: {}\n{}",
                    status, err_body
                ))
            }
        }
        Err(e) => {
            if e.is_timeout() {
                Err("Request to Ollama timed out. The model might be large or the server is overloaded.".to_string())
            } else if e.is_connect() {
                Err(
                    "Failed to connect to Ollama. Make sure it's running on the correct URL."
                        .to_string(),
                )
            } else {
                Err(format!("Error connecting to Ollama: {}", e))
            }
        }
    }
}
