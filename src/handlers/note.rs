use crate::note_handler;

pub struct NoteHandler {}

impl NoteHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_note(&self, content: &str) {
        note_handler::add_note(content);
    }

    pub fn show_recent_notes(&self, count: usize) {
        note_handler::show_recent_notes(count);
    }

    pub fn create_from_template(&self, template_type: &str) {
        note_handler::create_from_template(template_type);
    }
}
