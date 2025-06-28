use crate::cli::{Commands, PersonalizeAction};
use crate::handlers::{
    chat::ChatHandler, help::HelpHandler, interactive::InteractiveHandler, note::NoteHandler,
    personalization::PersonalizationHandler, setup::SetupHandler, suggestion::SuggestionHandler,
    system::SystemHandler,
};

pub struct CommandRouter {
    chat_handler: ChatHandler,
    help_handler: HelpHandler,
    interactive_handler: InteractiveHandler,
    note_handler: NoteHandler,
    personalization_handler: PersonalizationHandler,
    setup_handler: SetupHandler,
    suggestion_handler: SuggestionHandler,
    system_handler: SystemHandler,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            chat_handler: ChatHandler::new(),
            help_handler: HelpHandler::new(),
            interactive_handler: InteractiveHandler::new(),
            note_handler: NoteHandler::new(),
            personalization_handler: PersonalizationHandler::new(),
            setup_handler: SetupHandler::new(),
            suggestion_handler: SuggestionHandler::new(),
            system_handler: SystemHandler::new(),
        }
    }

    pub fn route(&self, command: Commands) {
        match command {
            // Setup and onboarding
            Commands::Setup => self.setup_handler.run_setup(),

            // Note-related commands
            Commands::Note { content } | Commands::N { content } => {
                self.note_handler.add_note(&content);
            }
            Commands::Recent { count } => {
                self.note_handler.show_recent_notes(count);
            }

            // AI interaction commands
            Commands::Suggestion { query } | Commands::S { query } => {
                self.suggestion_handler.get_suggestions(&query);
            }
            Commands::Chat { message } | Commands::C { message } => {
                self.chat_handler.chat_with_assistant(&message);
            }

            // Interactive mode
            Commands::Interactive => {
                self.interactive_handler.run();
            }

            // Personalization commands
            Commands::Personalize { action } => {
                match action {
                    Some(PersonalizeAction::Setup) => {
                        self.personalization_handler.setup_personalization();
                    }
                    Some(PersonalizeAction::Update) => {
                        self.personalization_handler.update_personalization();
                    }
                    Some(PersonalizeAction::Show) => {
                        self.personalization_handler.show_personalization();
                    }
                    Some(PersonalizeAction::Feedback { category }) => {
                        self.personalization_handler.feedback(category);
                    }
                    None => {
                        // Default to showing current personalization
                        self.personalization_handler.show_personalization();
                    }
                }
            }

            // Information and help commands
            Commands::About => self.help_handler.print_about(),
            Commands::How => self.help_handler.print_how(),
            Commands::Models => self.help_handler.print_models(),
            Commands::Context => self.help_handler.print_context(),
            Commands::Guide => self.help_handler.print_guide(),
            Commands::Troubleshoot => self.help_handler.print_troubleshoot(),

            // System commands
            Commands::Stats => self.system_handler.print_stats(),
            Commands::Doctor => self.system_handler.run_doctor(),
            Commands::Init => self.system_handler.run_init(),
            Commands::Completions { shell } => {
                self.system_handler.generate_completions(&shell);
            }
        }
    }
}
