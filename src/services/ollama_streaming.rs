use colored::*;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde_json::Value;
use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

/// Default timeout for Ollama streaming requests (300 seconds for large models and complex context-aware responses)
const OLLAMA_STREAMING_TIMEOUT: Duration = Duration::from_secs(300);

/// Streaming response handler for Ollama API
pub struct StreamingHandler {
    client: Client,
}

impl StreamingHandler {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Stream AI response with visual indicators
    pub async fn stream_suggestion(
        &self,
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

        // Show thinking indicator
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("AI is thinking...");
        pb.enable_steady_tick(Duration::from_millis(100));

        // Prepare the streaming request
        let ollama_body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        });

        // Debug: Print request info for troubleshooting
        if prompt.len() > 5000 {
            eprintln!(
                "Warning: Large prompt ({} chars) may cause slow response",
                prompt.len()
            );
        }

        let response = self
            .client
            .post(ollama_url)
            .timeout(OLLAMA_STREAMING_TIMEOUT)
            .header("Content-Type", "application/json")
            .json(&ollama_body)
            .send()
            .await;

        let response = match response {
            Ok(resp) if resp.status().is_success() => resp,
            Ok(resp) => {
                pb.finish_and_clear();
                let status = resp.status();
                let err_body = resp.text().await.unwrap_or_default();
                if status.as_u16() == 404 {
                    return Err(format!(
                        "Model '{model}' not found. Try: ollama pull {model}"
                    ));
                } else {
                    return Err(format!(
                        "Ollama server returned error status: {status}\n{err_body}"
                    ));
                }
            }
            Err(e) => {
                pb.finish_and_clear();
                if e.is_timeout() {
                    return Err("Request to Ollama timed out. The model might be large or the server is overloaded.".to_string());
                } else if e.is_connect() {
                    return Err(
                        "Failed to connect to Ollama. Make sure it's running on the correct URL."
                            .to_string(),
                    );
                } else {
                    return Err(format!("Error connecting to Ollama: {e}"));
                }
            }
        };

        // Start streaming
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        let mut final_response = String::new();
        let mut first_chunk = true;
        let mut successful_chunks = 0;
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 3;

        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    // Reset retry count on successful chunk
                    retry_count = 0;

                    // Handle potential UTF-8 encoding issues more gracefully
                    let chunk_str = match String::from_utf8(bytes.to_vec()) {
                        Ok(s) => s,
                        Err(e) => {
                            // If UTF-8 conversion fails, try with lossy conversion
                            eprintln!(
                                "Warning: UTF-8 conversion failed, using lossy conversion: {e}"
                            );
                            String::from_utf8_lossy(&bytes).to_string()
                        }
                    };

                    buffer.push_str(&chunk_str);

                    // Process complete lines
                    while let Some(newline_pos) = buffer.find('\n') {
                        let line = buffer[..newline_pos].trim().to_string();
                        buffer = buffer[newline_pos + 1..].to_string();

                        if !line.is_empty() {
                            match serde_json::from_str::<Value>(&line) {
                                Ok(data) => {
                                    // Check for errors in streaming response
                                    if let Some(error) = data.get("error").and_then(|v| v.as_str())
                                    {
                                        pb.finish_and_clear();
                                        return Err(format!("Ollama error: {error}"));
                                    }

                                    // Handle response chunk
                                    if let Some(response_chunk) =
                                        data.get("response").and_then(|v| v.as_str())
                                    {
                                        if first_chunk {
                                            // Clear thinking indicator and start response
                                            pb.finish_with_message("✅ AI responding...");
                                            println!(); // Add space after indicator
                                            first_chunk = false;
                                        }

                                        // Add realistic typing effect - stream character by character
                                        for char in response_chunk.chars() {
                                            print!("{}", char.to_string().bright_white());
                                            io::stdout().flush().unwrap();

                                            // Add small delay to simulate typing speed
                                            // Faster for spaces and punctuation, slower for letters
                                            let delay_ms = match char {
                                                ' ' => 20,                                // Fast for spaces
                                                '.' | ',' | '!' | '?' | ':' | ';' => 150, // Pause at punctuation
                                                '\n' => 200, // Longer pause for new lines
                                                _ => 25,     // Normal typing speed for letters
                                            };

                                            sleep(Duration::from_millis(delay_ms)).await;
                                        }

                                        final_response.push_str(response_chunk);
                                        successful_chunks += 1;
                                    }

                                    // Check if this is the final chunk
                                    if let Some(done) = data.get("done").and_then(|v| v.as_bool()) {
                                        if done {
                                            break;
                                        }
                                    }
                                }
                                Err(json_err) => {
                                    // More robust JSON error handling for malformed chunks
                                    if line.len() > 1000 {
                                        eprintln!("Warning: Large malformed JSON line ({} chars), skipping", line.len());
                                    } else {
                                        eprintln!("Warning: Could not parse JSON line: {line}");
                                        eprintln!("JSON error: {json_err}");
                                    }

                                    // For incomplete JSON that might be cut off, try to salvage partial data
                                    if line.contains("\"response\":") {
                                        if let Some(start) = line.find("\"response\":\"") {
                                            let start_pos = start + 12; // Skip past "response":"
                                            if let Some(end) = line[start_pos..].find("\"") {
                                                let partial_response =
                                                    &line[start_pos..start_pos + end];
                                                if !partial_response.is_empty() {
                                                    if first_chunk {
                                                        pb.finish_with_message(
                                                            "✅ AI responding...",
                                                        );
                                                        println!();
                                                        first_chunk = false;
                                                    }
                                                    print!("{}", partial_response.bright_white());
                                                    io::stdout().flush().unwrap();
                                                    final_response.push_str(partial_response);
                                                    successful_chunks += 1;
                                                }
                                            }
                                        }
                                    }
                                    // Continue processing other lines instead of failing
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count <= MAX_RETRIES {
                        eprintln!(
                            "Warning: Stream error (attempt {retry_count}/{MAX_RETRIES}): {e}"
                        );
                        eprintln!("Retrying stream read...");
                        // Small delay before retry
                        sleep(Duration::from_millis(100)).await;
                        continue;
                    } else {
                        pb.finish_and_clear();
                        return Err(format!(
                            "Error reading stream after {MAX_RETRIES} retries: {e}"
                        ));
                    }
                }
            }
        }

        // Process any remaining buffer content (in case stream ended mid-JSON)
        if !buffer.trim().is_empty() {
            let remaining_line = buffer.trim();
            if remaining_line.contains("\"response\":") {
                if let Some(start) = remaining_line.find("\"response\":\"") {
                    let start_pos = start + 12; // Skip past "response":"
                    if let Some(end) = remaining_line[start_pos..].find("\"") {
                        let partial_response = &remaining_line[start_pos..start_pos + end];
                        if !partial_response.is_empty() {
                            if first_chunk {
                                pb.finish_with_message("✅ AI responding...");
                                println!();
                                first_chunk = false;
                            }
                            print!("{}", partial_response.bright_white());
                            io::stdout().flush().unwrap();
                            final_response.push_str(partial_response);
                            successful_chunks += 1;
                        }
                    }
                }
            }
        }

        // Add final newline after response
        if !first_chunk {
            println!();
        } else {
            pb.finish_and_clear();
        }

        if final_response.trim().is_empty() {
            if successful_chunks == 0 {
                Err("No valid response received from Ollama. The model might not be loaded or compatible.".to_string())
            } else {
                Err("Model generated empty response. Try a different prompt or model.".to_string())
            }
        } else {
            Ok(final_response)
        }
    }

    /// Stream chat response with enhanced visual feedback
    pub async fn stream_chat_response(
        &self,
        ollama_url: &str,
        model: &str,
        prompt: &str,
    ) -> Result<String, String> {
        // Just use the regular streaming function directly - no fancy delays that cause infinite loops
        self.stream_suggestion(ollama_url, model, prompt).await
    }
}

impl Default for StreamingHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for backward compatibility - now uses streaming
pub async fn generate_suggestion_streaming(
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let handler = StreamingHandler::new();
    handler.stream_suggestion(ollama_url, model, prompt).await
}

/// Convenience function for chat responses with enhanced feedback
pub async fn generate_chat_streaming(
    ollama_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let handler = StreamingHandler::new();
    handler
        .stream_chat_response(ollama_url, model, prompt)
        .await
}
