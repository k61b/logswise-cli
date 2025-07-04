use crate::cli::{Commands, PersonalizeAction};
use crate::handlers::{
    personalization::PersonalizationHandler, setup::SetupHandler, system::SystemHandler,
};

pub struct CommandRouter {
    personalization_handler: PersonalizationHandler,
    setup_handler: SetupHandler,
    system_handler: SystemHandler,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            personalization_handler: PersonalizationHandler::new(),
            setup_handler: SetupHandler::new(),
            system_handler: SystemHandler::new(),
        }
    }

    pub async fn route(&mut self, command: Commands) {
        match command {
            // Setup and onboarding
            Commands::Setup {
                update,
                express,
                template,
                import,
            } => {
                self.setup_handler
                    .run_smart_setup(update, express, template, import)
                    .await;
            }

            // Main interactive interface
            Commands::Simple => {
                if let Err(e) = crate::simple_interactive::run_simple_interactive().await {
                    eprintln!("Interactive mode error: {}", e);
                }
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

            // System commands
            Commands::Init => self.system_handler.run_init().await,
        }
    }
}
