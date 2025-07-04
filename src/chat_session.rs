use colored::*;
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::config::{get_ollama_config, load_profile};
use crate::services::ollama_streaming::generate_chat_streaming;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,    // "user" or "assistant"
    pub content: String, // The message content
    pub timestamp: u64,  // Unix timestamp
}

impl ChatMessage {
    pub fn new_user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn new_assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

pub struct ChatSession {
    conversation_history: VecDeque<ChatMessage>,
    max_history_length: usize,
    max_context_tokens: usize,
    profile: serde_json::Value,
    session_start_time: u64,
}

impl ChatSession {
    pub fn new() -> Result<Self, String> {
        let profile = load_profile().map_err(|e| format!("Error loading profile: {e}"))?;

        Ok(Self {
            conversation_history: VecDeque::new(),
            max_history_length: 10, // Keep last 10 messages for context (reduced from 20)
            max_context_tokens: 2000, // Reduced token limit for faster processing
            profile,
            session_start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    pub async fn start_interactive_session(&mut self) -> Result<(), String> {
        println!("{}", "💬 Chat Session Started".cyan().bold());
        println!(
            "{}",
            "Type your messages below. Commands: '/history', '/clear', '/stats', '/help', '/exit'"
                .bright_black()
        );
        println!("{}", "─".repeat(70).bright_black());
        println!();

        loop {
            // Get user input
            let user_input = match Input::<String>::new().with_prompt("You").interact_text() {
                Ok(input) => input.trim().to_string(),
                Err(_) => {
                    println!("{}", "❌ Input error. Exiting chat session.".red());
                    break;
                }
            };

            if user_input.is_empty() {
                continue;
            }

            // Handle special commands
            match user_input.as_str() {
                "/exit" | "/quit" | "exit" | "quit" => {
                    println!("{}", "👋 Chat session ended. Goodbye!".green());
                    break;
                }
                "/history" => {
                    self.show_conversation_history();
                    continue;
                }
                "/clear" => {
                    self.clear_history();
                    println!("{}", "🧹 Conversation history cleared.".yellow());
                    continue;
                }
                "/help" => {
                    self.show_chat_help();
                    continue;
                }
                "/stats" => {
                    self.show_session_stats();
                    continue;
                }
                _ => {}
            }

            // Process the user message
            match self.process_user_message(user_input).await {
                Ok(_) => {
                    // Success - continue conversation
                    println!(); // Add spacing
                }
                Err(e) => {
                    println!("{}", format!("❌ Error: {e}").red());
                    println!("{}", "Try again or type '/exit' to quit.".bright_black());
                }
            }
        }

        Ok(())
    }

    async fn process_user_message(&mut self, user_message: String) -> Result<(), String> {
        // Add user message to history
        self.add_message(ChatMessage::new_user(user_message.clone()));

        // Build conversation context for the AI
        let conversation_context = self.build_conversation_context();

        // Get Ollama configuration
        let (ollama_url, ollama_model) = get_ollama_config(&self.profile);

        // Show assistant typing indicator
        print!("{}", "🤖 Assistant: ".bright_green().bold());

        // Generate response using streaming
        let response =
            generate_chat_streaming(&ollama_url, &ollama_model, &conversation_context).await?;

        // Add assistant response to history
        self.add_message(ChatMessage::new_assistant(response));

        Ok(())
    }

    fn build_conversation_context(&self) -> String {
        let profession = self
            .profile
            .get("profession")
            .and_then(|v| v.as_str())
            .unwrap_or("Professional");

        // Keep it simple and short for fast processing
        let mut context = format!(
            "You are a helpful AI assistant chatting with a {profession}. Be conversational and helpful.\n\n"
        );

        // Only include the last few messages to keep prompt short
        if !self.conversation_history.is_empty() {
            context.push_str("Recent conversation:\n");
            // Only take the last 4 messages to keep prompt manageable
            let recent_messages: Vec<_> = self
                .conversation_history
                .iter()
                .rev()
                .take(4)
                .rev()
                .collect();

            for message in recent_messages {
                // Truncate long messages to keep prompt short
                let truncated_content = if message.content.len() > 100 {
                    format!("{}...", &message.content[..100])
                } else {
                    message.content.clone()
                };

                context.push_str(&format!(
                    "{}: {}\n",
                    if message.role == "user" {
                        "User"
                    } else {
                        "Assistant"
                    },
                    truncated_content
                ));
            }
            context.push_str("\nRespond to the latest message:");
        } else {
            context.push_str("Respond in a friendly, conversational way:");
        }

        context
    }

    fn add_message(&mut self, message: ChatMessage) {
        self.conversation_history.push_back(message);

        // Maintain max history length
        while self.conversation_history.len() > self.max_history_length {
            self.conversation_history.pop_front();
        }

        // Also trim by approximate token count if needed
        self.trim_by_tokens();
    }

    fn trim_by_tokens(&mut self) {
        let mut total_chars = 0;
        let approximate_chars_per_token = 4; // Rough approximation
        let max_chars = self.max_context_tokens * approximate_chars_per_token;

        // Count characters from most recent messages backwards
        let mut keep_count = 0;
        for message in self.conversation_history.iter().rev() {
            let message_chars = message.content.len();
            if total_chars + message_chars > max_chars && keep_count > 2 {
                // Always keep at least 2 messages for context
                break;
            }
            total_chars += message_chars;
            keep_count += 1;
        }

        // Remove oldest messages if we exceed the limit
        while self.conversation_history.len() > keep_count {
            self.conversation_history.pop_front();
        }
    }

    fn show_conversation_history(&self) {
        if self.conversation_history.is_empty() {
            println!("{}", "📝 No conversation history yet.".yellow());
            return;
        }

        println!("{}", "📜 Conversation History:".cyan().bold());
        println!("{}", "─".repeat(50).bright_black());

        for (i, message) in self.conversation_history.iter().enumerate() {
            let role_label = if message.role == "user" {
                "You".bright_blue().bold()
            } else {
                "🤖 Assistant".bright_green().bold()
            };

            // Truncate long messages for history display
            let content = if message.content.len() > 80 {
                format!("{}...", &message.content[0..80])
            } else {
                message.content.clone()
            };

            println!("{}. {}: {}", i + 1, role_label, content);
        }

        println!("{}", "─".repeat(50).bright_black());
        println!();
    }

    fn clear_history(&mut self) {
        self.conversation_history.clear();
    }

    fn show_chat_help(&self) {
        println!("{}", "💬 Chat Session Help".cyan().bold());
        println!("{}", "─".repeat(40).bright_black());
        println!("{} - Show conversation history", "/history".green());
        println!("{} - Clear conversation history", "/clear".green());
        println!("{} - Show session statistics", "/stats".green());
        println!("{} - Show this help message", "/help".green());
        println!("{} - Exit chat session", "/exit".green());
        println!();
        println!("{}", "💡 Tips:".yellow().bold());
        println!("• The AI remembers your conversation context");
        println!("• Your messages are kept in memory during the session");
        println!("• Use natural language - ask follow-up questions!");
        println!("• Long conversations are automatically trimmed for performance");
        println!("{}", "─".repeat(40).bright_black());
        println!();
    }

    fn show_session_stats(&self) {
        println!("{}", "📊 Chat Session Statistics".cyan().bold());
        println!("{}", "─".repeat(40).bright_black());

        let session_duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - self.session_start_time;

        let duration_minutes = session_duration / 60;
        let duration_seconds = session_duration % 60;

        let total_chars: usize = self
            .conversation_history
            .iter()
            .map(|msg| msg.content.len())
            .sum();

        let user_messages = self
            .conversation_history
            .iter()
            .filter(|msg| msg.role == "user")
            .count();

        let assistant_messages = self
            .conversation_history
            .iter()
            .filter(|msg| msg.role == "assistant")
            .count();

        println!("• Session Duration: {duration_minutes}m {duration_seconds}s");
        println!("• Total Messages: {}", self.conversation_history.len());
        println!("• Your Messages: {user_messages}");
        println!("• Assistant Messages: {assistant_messages}");
        println!("• Total Characters: {total_chars}");
        println!(
            "• Memory Usage: {}/{} messages",
            self.conversation_history.len(),
            self.max_history_length
        );
        println!(
            "• Estimated Tokens: ~{}/{}",
            total_chars / 4,
            self.max_context_tokens
        );

        println!("{}", "─".repeat(40).bright_black());
        println!();
    }

    #[allow(dead_code)]
    pub fn get_history_length(&self) -> usize {
        self.conversation_history.len()
    }

    #[allow(dead_code)]
    pub fn get_last_messages(&self, count: usize) -> Vec<&ChatMessage> {
        self.conversation_history
            .iter()
            .rev()
            .take(count)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}

impl Default for ChatSession {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            conversation_history: VecDeque::new(),
            max_history_length: 20,
            max_context_tokens: 4000,
            profile: serde_json::json!({}),
            session_start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        })
    }
}

/// Start an interactive chat session
pub async fn start_chat_session() -> Result<(), String> {
    let mut session = ChatSession::new()?;
    session.start_interactive_session().await
}
