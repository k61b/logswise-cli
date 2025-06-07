use reqwest::blocking::Client;
use serde_json::Value;

pub fn generate_embedding(
    client: &Client,
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<Vec<f32>, String> {
    let embedding_body = serde_json::json!({
        "model": model,
        "prompt": prompt
    });
    let embedding_res = client
        .post(ollama_url)
        .header("Content-Type", "application/json")
        .json(&embedding_body)
        .send();
    match embedding_res {
        Ok(resp) if resp.status().is_success() => {
            let resp_text = resp.text().unwrap_or_default();
            let resp_json: Value = serde_json::from_str(&resp_text).unwrap_or_default();
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
                Err(format!("Unexpected embedding response: {}", resp_text))
            }
        }
        Ok(resp) => {
            let status = resp.status();
            let err_text = resp.text().unwrap_or_default();
            Err(format!(
                "Ollama returned error status {}: {}",
                status, err_text
            ))
        }
        Err(e) => Err(format!("Failed to connect to Ollama: {}", e)),
    }
}

pub fn generate_suggestion(
    client: &Client,
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let ollama_body = serde_json::json!({
        "model": model,
        "prompt": prompt
    });
    let ollama_res = client
        .post(ollama_url)
        .header("Content-Type", "application/json")
        .json(&ollama_body)
        .send();
    match ollama_res {
        Ok(resp) if resp.status().is_success() => {
            let raw_body = resp.text().unwrap_or_default();
            let mut final_response = String::new();
            for line in raw_body.lines() {
                if let Ok(data) = serde_json::from_str::<Value>(line) {
                    if let Some(resp_str) = data.get("response").and_then(|v| v.as_str()) {
                        final_response.push_str(resp_str);
                    }
                }
            }
            Ok(final_response)
        }
        Ok(resp) => {
            let status = resp.status();
            let err_body = resp.text().unwrap_or_default();
            Err(format!(
                "Ollama server returned error status: {}\n{}",
                status, err_body
            ))
        }
        Err(e) => Err(format!("Error connecting to Ollama: {}", e)),
    }
}
