use crate::interactive;

pub struct InteractiveHandler {}

impl InteractiveHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        interactive::run_interactive();
    }
}
