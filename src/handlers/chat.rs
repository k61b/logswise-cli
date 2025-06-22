use crate::chat_handler;

pub struct ChatHandler {}

impl ChatHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn chat_with_assistant(&self, message: &str) {
        chat_handler::chat_with_assistant(message);
    }
}
