//! Simplified template system for prompts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct PromptContext {
    #[allow(dead_code)]
    pub variables: HashMap<String, String>,
}

impl PromptContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }
}

impl Default for PromptContext {
    fn default() -> Self {
        Self::new()
    }
}
