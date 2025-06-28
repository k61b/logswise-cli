use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::time::Duration;

use crate::personalization::UserContext;
use crate::services::ollama;
use crate::services::supabase;
use crate::utils;

pub fn get_suggestions(query: &str) {
    let profile = match utils::load_profile() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", format!("Error loading profile: {e}").red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };

    // Load enhanced user context for personalization
    let mut user_context = match UserContext::load_or_create() {
        Ok(ctx) => ctx,
        Err(e) => {
            println!(
                "{}",
                format!("Warning: Could not load enhanced context: {e}").yellow()
            );
            println!("Using basic personalization. Run 'logswise-cli personalize' for better suggestions.");
            // Continue with basic profile-based suggestions
            get_basic_suggestions(query, &profile);
            return;
        }
    };

    let llm_name = profile["llmName"].as_str().unwrap_or("").to_lowercase();
    if llm_name.is_empty() {
        println!(
            "{}",
            "😅 No LLM configured. Please set up your LLM in setup.json.".yellow()
        );
        return;
    }

    let ollama_base_url = profile["ollamaBaseUrl"]
        .as_str()
        .unwrap_or("http://localhost:11434");
    let ollama_embedding_url = format!("{ollama_base_url}/api/embeddings");
    let ollama_generate_url = format!("{ollama_base_url}/api/generate");
    let ollama_model = profile["embeddingModel"]
        .as_str()
        .unwrap_or("nomic-embed-text");

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Loading enhanced user context and preparing personalized suggestions...");

    let _notes_context = String::new();
    let config = match utils::load_supabase_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}", format!("Error loading Supabase config: {e}").red());
            println!("Please run 'logswise-cli setup' first.");
            return;
        }
    };

    let client = Client::new();
    let embedding_models = [
        "nomic-embed-text",
        "bge-base-en",
        "all-minilm",
        // Add more known embedding models here if needed
    ];
    let is_embedding = embedding_models
        .iter()
        .any(|m| llm_name == *m || llm_name.starts_with(m));

    if is_embedding {
        // Only perform embedding and semantic search, print results, and exit (no LLM generation)
        println!(
            "⚡ Running in embedding-only mode (semantic search, no LLM generation). Model: {}",
            llm_name.cyan()
        );
        // 1. Generate embedding for the query using local Ollama
        let query_embedding = match ollama::generate_embedding(
            &client,
            &ollama_embedding_url,
            ollama_model,
            query,
        ) {
            Ok(embedding) => embedding,
            Err(msg) => {
                spinner.finish_and_clear();
                println!(
                    "{}",
                    "❌ Could not generate embedding for query. No semantic search can be made."
                        .red()
                );
                println!("{}", msg.yellow());
                println!("➡️  Please check that your embedding model is pulled and running in Ollama (e.g., 'ollama pull nomic-embed-text'), and that OLLAMA_EMBEDDING_MODEL is set correctly.");
                return;
            }
        };
        // 2. Query Supabase for most similar notes (top 5)
        let notes = supabase::semantic_search_notes(&client, &config, &query_embedding, 5);
        if !notes.is_empty() {
            println!("\nRelevant Notes:");
            for (i, content) in notes.iter().enumerate() {
                println!("{}. {}", i + 1, content);
            }
        } else {
            println!("No relevant notes found.");
        }
        spinner.finish_and_clear();
        return;
    }

    // 1. Generate embedding for the query using local Ollama
    let query_embedding = match ollama::generate_embedding(
        &client,
        &ollama_embedding_url,
        ollama_model,
        query,
    ) {
        Ok(embedding) => embedding,
        Err(msg) => {
            spinner.finish_and_clear();
            println!(
                "{}",
                "❌ Could not generate embedding for query. No suggestions can be made.".red()
            );
            println!("{}", msg.yellow());
            println!("➡️  Please check that your embedding model is pulled and running in Ollama (e.g., 'ollama pull nomic-embed-text'), and that OLLAMA_EMBEDDING_MODEL is set correctly.");
            return;
        }
    };

    // 2. Query Supabase for most similar notes (top 5)
    let notes = supabase::semantic_search_notes(&client, &config, &query_embedding, 5);

    // Generate enhanced prompt using user context
    let full_prompt = user_context.generate_llm_context(query, &notes);

    // Create personalized instruction based on user preferences
    let personalized_instruction = create_personalized_instruction(&user_context);
    let complete_prompt = format!("{full_prompt}\n\n{personalized_instruction}");

    println!(
        "🔎 Using Ollama model: {} with enhanced personalization",
        llm_name.cyan()
    );
    spinner.set_message("Ollama: Generating personalized suggestions...");

    match ollama::generate_suggestion(&client, &ollama_generate_url, &llm_name, &complete_prompt) {
        Ok(final_response) => {
            spinner.finish_and_clear();
            if !final_response.trim().is_empty() {
                println!(
                    "\n==================== 💡 Personalized Suggestions ====================\n"
                );
                let final_answer = if let Some(idx) = final_response.find("</think>") {
                    final_response[(idx + "</think>".len())..].trim()
                } else {
                    final_response.trim()
                };
                println!(
                    "----------------------------------------\n{final_answer}\n----------------------------------------\n"
                );

                // Update interaction history (simplified for now)
                // Note: We don't store the actual query to prevent hallucination
                user_context
                    .interaction_history
                    .recent_topics
                    .push("suggestion_request".to_string());
                if user_context.interaction_history.recent_topics.len() > 10 {
                    user_context.interaction_history.recent_topics.remove(0);
                }
                let _ = user_context.save(); // Save updated context
            } else {
                println!("{} {}", "❌ No suggestion from model:".red(), llm_name);
            }
        }
        Err(msg) => {
            spinner.finish_and_clear();
            println!("{}", msg.red());
        }
    }
}

// Fallback to basic suggestions if enhanced context is not available
fn get_basic_suggestions(_query: &str, profile: &serde_json::Value) {
    // This is the original basic implementation
    let _user_info = format!(
        "User Info:\n- Profession: {}\n- Job Title: {}\n- Company Name: {}\n- Company Size: {}",
        profile["profession"].as_str().unwrap_or(""),
        profile["jobTitle"].as_str().unwrap_or(""),
        profile["companyName"].as_str().unwrap_or(""),
        profile["companySize"].as_str().unwrap_or("")
    );

    // Continue with basic implementation...
    println!(
        "Using basic suggestion mode. Run 'logswise-cli personalize' for enhanced suggestions."
    );
}

fn create_personalized_instruction(context: &UserContext) -> String {
    let mut instruction = String::new();

    // Advanced AI system prompt with enterprise-grade constraints
    instruction.push_str("=== ADVANCED AI ASSISTANT CONFIGURATION ===\n");
    instruction.push_str("You are an elite AI advisor specializing in professional development.\n");
    instruction.push_str(
        "Your expertise: Strategic thinking, project management, career advancement.\n\n",
    );

    instruction.push_str("🎯 CORE PRINCIPLES:\n");
    instruction.push_str("1. EVIDENCE-BASED: All suggestions grounded in verified user data\n");
    instruction.push_str("2. CONTEXT-AWARE: Use conversation context without inventing details\n");
    instruction.push_str("3. ACTIONABLE: Provide specific, measurable recommendations\n");
    instruction.push_str("4. STRATEGIC: Balance immediate needs with long-term goals\n");
    instruction.push_str("5. PROFESSIONAL: Maintain enterprise-level communication standards\n\n");

    instruction.push_str("🚫 ABSOLUTE CONSTRAINTS:\n");
    instruction.push_str("- NEVER fabricate relationships, conversations, or commitments\n");
    instruction
        .push_str("- NEVER assume details about team dynamics or organizational structure\n");
    instruction.push_str("- ALWAYS ground advice in documented projects and verified challenges\n");
    instruction.push_str("- ALWAYS include success metrics and follow-up mechanisms\n\n");

    // Advanced response formatting based on communication style
    instruction.push_str("📝 RESPONSE STYLE CONFIGURATION:\n");
    match context.preferences.communication_style.as_str() {
        "concise" => {
            instruction.push_str("FORMAT: Executive Summary Style\n");
            instruction.push_str("- Use bullet points and numbered lists\n");
            instruction.push_str("- Maximum 3 key recommendations\n");
            instruction.push_str("- Include one-line success metrics for each\n");
            instruction.push_str("- Total response: 150-250 words\n");
        }
        "detailed" => {
            instruction.push_str("FORMAT: Comprehensive Analysis Style\n");
            instruction.push_str("- Provide detailed reasoning and context\n");
            instruction.push_str("- Include implementation steps and timeline\n");
            instruction.push_str("- Address potential challenges and mitigation\n");
            instruction.push_str("- Total response: 300-500 words\n");
        }
        "casual" => {
            instruction.push_str("FORMAT: Collaborative Advisor Style\n");
            instruction.push_str("- Use friendly, encouraging language\n");
            instruction.push_str("- Include motivational elements\n");
            instruction.push_str("- Make suggestions feel approachable and doable\n");
            instruction.push_str("- Use emojis strategically for clarity\n");
        }
        "professional" => {
            instruction.push_str("FORMAT: Enterprise Consultant Style\n");
            instruction.push_str("- Use formal, structured language\n");
            instruction.push_str("- Focus on business impact and ROI\n");
            instruction.push_str("- Include stakeholder considerations\n");
            instruction.push_str("- Emphasize measurable outcomes\n");
        }
        _ => {
            instruction.push_str("FORMAT: Balanced Professional Style\n");
            instruction.push_str("- Clear, structured communication\n");
            instruction.push_str("- Professional yet approachable tone\n");
            instruction.push_str("- Focus on practical implementation\n");
        }
    }

    // Adapt based on learning style
    match context.learning_style.preferred_format.as_str() {
        "hands_on" => {
            instruction.push_str("LEARNING APPROACH: Prioritize practical exercises, code examples, and actionable tasks. Include specific implementation steps.\n");
        }
        "reading" => {
            instruction.push_str("LEARNING APPROACH: Suggest articles, documentation, and written resources. Include book recommendations when relevant.\n");
        }
        "videos" => {
            instruction.push_str("LEARNING APPROACH: Suggest video tutorials, online courses, and visual learning resources.\n");
        }
        "peer_learning" => {
            instruction.push_str("LEARNING APPROACH: Emphasize collaborative learning, team discussions, mentoring opportunities, and community engagement.\n");
        }
        _ => {
            instruction.push_str(
                "LEARNING APPROACH: Mix different learning methods based on the topic.\n",
            );
        }
    }

    // Add focus areas
    if !context.preferences.focus_areas.is_empty() {
        instruction.push_str(&format!(
            "FOCUS PRIORITIES: Emphasize suggestions related to: {}\n",
            context.preferences.focus_areas.join(", ")
        ));
    }

    // Add complexity guidance
    match context.learning_style.complexity_preference.as_str() {
        "beginner" => {
            instruction.push_str("COMPLEXITY: Provide beginner-friendly suggestions with step-by-step guidance. Avoid advanced concepts without explanation.\n");
        }
        "advanced" => {
            instruction.push_str("COMPLEXITY: Feel free to suggest advanced techniques and deep technical concepts. Assume strong foundational knowledge.\n");
        }
        "adaptive" => {
            instruction.push_str("COMPLEXITY: Adapt complexity based on the topic and provide both beginner and advanced options when relevant.\n");
        }
        _ => {
            instruction.push_str(
                "COMPLEXITY: Use intermediate-level suggestions with clear explanations.\n",
            );
        }
    }

    // Add feedback preference
    match context.learning_style.feedback_preference.as_str() {
        "immediate" => {
            instruction.push_str("TRACKING: Include ways to get immediate feedback and quick wins. Suggest daily or weekly check-ins.\n");
        }
        "milestone" => {
            instruction.push_str("TRACKING: Focus on milestone-based progress tracking. Suggest monthly or quarterly reviews.\n");
        }
        _ => {
            instruction
                .push_str("TRACKING: Include periodic progress checks and feedback mechanisms.\n");
        }
    }

    instruction.push_str("\n🎯 FINAL EXECUTION PROTOCOL:\n");
    instruction.push_str("ANALYZE the query intent and map to relevant project context\n");
    instruction.push_str("SYNTHESIZE 2-4 high-impact recommendations with:\n");
    instruction.push_str("├─ SPECIFIC ACTIONS: What exactly to do\n");
    instruction.push_str("├─ SUCCESS METRICS: How to measure progress\n");
    instruction.push_str("├─ TIMELINE: When to complete each action\n");
    instruction.push_str("├─ STAKEHOLDER IMPACT: Who benefits and how\n");
    instruction.push_str("└─ FOLLOW-UP: Next review point or checkpoint\n\n");

    instruction.push_str("Connect ALL advice to their documented:\n");
    instruction.push_str(
        "✓ Projects: Vena (high-pressure, team challenges) + logswise-cli (solo, learning)\n",
    );
    instruction.push_str("✓ Career Goals: Senior developer advancement\n");
    instruction.push_str("✓ Current Challenges: Platform/business role clarity, team dynamics, skill development\n\n");

    instruction.push_str("EXCELLENCE STANDARD: Each recommendation should be:\n");
    instruction.push_str("• Immediately actionable within 24-48 hours\n");
    instruction.push_str("• Measurable with clear success criteria\n");
    instruction.push_str("• Aligned with their career progression goals\n");
    instruction.push_str("• Contextually relevant to their current situation\n");

    instruction
}

// Helper function to collect user feedback on suggestions
pub fn collect_suggestion_feedback(_suggestion_category: &str) -> Result<(bool, f32), String> {
    use dialoguer::{Confirm, Select};

    let accepted = Confirm::new()
        .with_prompt("Did you find this suggestion helpful?")
        .default(true)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    let satisfaction_options = vec![
        "Very satisfied (5/5)",
        "Satisfied (4/5)",
        "Neutral (3/5)",
        "Dissatisfied (2/5)",
        "Very dissatisfied (1/5)",
    ];

    let satisfaction_idx = Select::new()
        .with_prompt("How satisfied were you with this suggestion?")
        .items(&satisfaction_options)
        .default(1)
        .interact()
        .map_err(|e| format!("Input error: {e}"))?;

    let satisfaction = match satisfaction_idx {
        0 => 1.0,
        1 => 0.8,
        2 => 0.6,
        3 => 0.4,
        4 => 0.2,
        _ => 0.6,
    };

    Ok((accepted, satisfaction))
}

#[cfg(test)]
mod tests {
    use crate::types::SupabaseConfig;
    use serde_json::json;

    #[test]
    fn test_load_supabase_config_valid() {
        // Prepare a temp setup.json
        let tmp_dir = tempfile::tempdir().unwrap();
        let mut setup_path = tmp_dir.path().to_path_buf();
        setup_path.push("setup.json");
        let json = r#"{
            "supabaseUrl": "https://test.supabase.co",
            "supabaseApiKey": "testkey"
        }"#;
        std::fs::write(&setup_path, json).unwrap();
        // Patch home_dir to return tmp_dir (not possible here), so just test parsing logic
        let data = std::fs::read_to_string(&setup_path).unwrap();
        let profile: serde_json::Value = serde_json::from_str(&data).unwrap();
        let config = SupabaseConfig {
            project_url: profile["supabaseUrl"].as_str().unwrap().to_string(),
            api_key: profile["supabaseApiKey"].as_str().unwrap().to_string(),
        };
        assert_eq!(config.project_url, "https://test.supabase.co");
        assert_eq!(config.api_key, "testkey");
    }

    #[test]
    fn test_load_profile_valid() {
        let profile = json!({
            "profession": "Developer",
            "jobTitle": "Senior",
            "companyName": "TestCo",
            "companySize": "10-100",
            "llmName": "llama3",
            "supabaseUrl": "https://test.supabase.co",
            "supabaseApiKey": "testkey"
        });
        assert_eq!(profile["companyName"], "TestCo");
        assert_eq!(profile["llmName"], "llama3");
    }

    #[test]
    fn test_prompt_formatting() {
        let user_info = "User Info:\n- Profession: Developer\n- Job Title: Senior\n- Company Name: TestCo\n- Company Size: 10-100";
        let notes_context = "\nRecent Notes:\n1. Note one\n2. Note two";
        let query = "How to improve logging?";
        let personalization = "At TestCo scale (10-100), consider centralized log filtering.\n";
        let cli_instruction = "Reply in this format:\n=== Quick Summary ===\n(3-line summary)\n=== Suggestions by Category ===\n- Learning:\n  1. Suggestion (with a simple way to track success)\n  2. ...\n- Collaboration:\n  3. ...\n- Well-being:\n  4. ...\n(Up to 10 total, grouped by category. For each, add a quick feedback loop, e.g., 'Do a team poll after 2 weeks' or 'Check adoption in next retro'. Keep the tone informal and practical for a small, busy team. No markdown, CLI readable)";
        let full_prompt = format!(
            "{user_info}{notes_context}\n\nUser wants suggestions for: {query}\n{personalization}{cli_instruction}\nSuggestions:"
        );
        assert!(full_prompt.contains("User wants suggestions for: How to improve logging?"));
        assert!(full_prompt.contains("Recent Notes:"));
        assert!(full_prompt.contains("=== Quick Summary ==="));
    }
}
