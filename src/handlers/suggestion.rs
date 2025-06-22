use crate::suggestion_handler;

pub struct SuggestionHandler {}

impl SuggestionHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_suggestions(&self, query: &str) {
        suggestion_handler::get_suggestions(query);
    }

    pub fn quick_suggestions(&self, suggestion_type: &str) {
        suggestion_handler::quick_suggestions(suggestion_type);
    }
}
