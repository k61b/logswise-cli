# Technical Documentation - Logswise CLI

## Project Structure

```
logswise-cli/
├── src/                          # Rust source code
│   ├── main.rs                   # Entry point, runtime setup
│   ├── cli.rs                    # Command-line argument parsing (clap)
│   ├── router.rs                 # Command routing logic
│   ├── simple_interactive.rs     # Main interactive UI loop
│   ├── types.rs                  # Shared data structures
│   ├── errors.rs                 # Error handling types
│   ├── utils.rs                  # Utility functions
│   ├── validation.rs             # Input validation
│   │
│   ├── config/                   # Configuration management
│   │   ├── mod.rs                # Module exports and utilities
│   │   ├── manager.rs            # Main configuration manager
│   │   ├── setup_modes.rs        # Different setup modes (express, template, etc.)
│   │   ├── templates.rs          # Role-based configuration templates
│   │   ├── updater.rs            # Configuration update logic
│   │   └── validator.rs          # Configuration validation
│   │
│   ├── handlers/                 # Command handlers
│   │   ├── mod.rs                # Handler exports
│   │   ├── setup.rs              # Setup command handler
│   │   ├── system.rs             # System commands (init, doctor)
│   │   ├── personalization.rs    # Personalization commands
│   │   └── help.rs               # Help system
│   │
│   ├── services/                 # External service integrations
│   │   ├── mod.rs                # Service exports
│   │   ├── config.rs             # Configuration service
│   │   ├── supabase.rs           # Supabase API client
│   │   ├── ollama.rs             # Ollama API client
│   │   ├── ollama_streaming.rs   # Ollama streaming responses
│   │   └── note_service.rs       # Note management service
│   │
│   ├── prompts/                  # AI prompt templates
│   │   ├── mod.rs                # Prompt system exports
│   │   ├── registry.rs           # Prompt registry
│   │   ├── template.rs           # Prompt template engine
│   │   ├── prompt_registry.yaml  # Prompt configuration
│   │   ├── ai-suggestions/       # AI suggestion prompts
│   │   ├── daily-logging/        # Daily logging prompts
│   │   ├── performance-analysis/ # Performance analysis prompts
│   │   └── common/               # Common prompt templates
│   │
│   ├── chat_handler_v3.rs        # Chat functionality
│   ├── chat_session.rs           # Chat session management
│   ├── note_handler.rs           # Note operations
│   ├── suggestion_handler_v3.rs  # AI suggestion generation
│   └── personalization.rs        # User personalization logic
│
├── docs/                         # Documentation website
│   ├── index.html                # Main documentation page
│   ├── script.js                 # Documentation functionality
│   └── style.css                 # Documentation styling
│
├── Cargo.toml                    # Rust dependencies and metadata
├── Cargo.lock                    # Locked dependency versions
├── README.md                     # Main project documentation
├── SUPABASE_SETUP.md            # Database setup instructions
├── CONTRIBUTING.md              # Contribution guidelines
├── LICENSE                      # AGPL v3.0 license
└── SECURITY.md                  # Security policy
```

## Core Dependencies

From `Cargo.toml`:

- **clap**: Command-line argument parsing with derive features
- **serde/serde_json**: JSON serialization/deserialization
- **reqwest**: HTTP client for API calls (blocking + streaming)
- **dialoguer**: Interactive prompts and menus
- **colored**: Terminal color output
- **figlet-rs**: ASCII art text generation
- **tokio**: Async runtime with full features
- **futures**: Async utilities
- **crossterm**: Cross-platform terminal manipulation
- **indicatif**: Progress bars and spinners
- **chrono**: Date/time handling
- **dirs**: System directory paths

## Data Flow

1. **CLI Entry** (`main.rs`): Parses arguments, creates runtime, routes commands
2. **Command Routing** (`router.rs`): Dispatches to appropriate handlers
3. **Interactive Mode** (`simple_interactive.rs`): Main UI loop with menu system
4. **Service Layer** (`services/`): Handles external API calls to Supabase/Ollama
5. **Configuration** (`config/`): Manages `~/.logswise/setup.json` file
6. **Prompts** (`prompts/`): Template system for AI interactions

## Database Schema

Supabase PostgreSQL with pgvector extension:

```sql
-- Main notes table
CREATE TABLE notes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  content TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()),
  embedding VECTOR(768)  -- For semantic search
);

-- pgvector extension for embeddings
CREATE EXTENSION IF NOT EXISTS vector;
```

## Configuration Format

Stored in `~/.logswise/setup.json`:

```json
{
  "supabase_url": "https://your-project.supabase.co",
  "supabase_key": "your-anon-key",
  "ollama_url": "http://localhost:11434",
  "model_name": "llama3",
  "user_profile": {
    "name": "Your Name",
    "role": "Developer",
    "preferences": { ... }
  },
  "personalization": {
    "communication_style": "direct",
    "complexity_level": "intermediate",
    "learning_style": "hands-on"
  }
}
```

## Key Modules

### `simple_interactive.rs`
- Main UI loop with menu system
- Handles user input and navigation
- Calls appropriate handlers based on user selection
- Manages screen clearing and formatting

### `chat_handler_v3.rs` / `chat_session.rs`
- Implements streaming chat with Ollama
- Maintains conversation context
- Integrates user notes as context for AI responses

### `suggestion_handler_v3.rs`
- Generates AI suggestions based on user query
- Uses vector similarity search on stored notes
- Formats suggestions with personalization

### `note_handler.rs`
- CRUD operations for notes
- Generates embeddings for semantic search
- Handles note storage in Supabase

### `config/manager.rs`
- Manages configuration file I/O
- Handles setup wizard logic
- Validates configuration parameters

### `services/supabase.rs`
- Supabase REST API client
- Handles authentication and requests
- Manages vector operations

### `services/ollama.rs`
- Ollama API client for text generation
- Handles both blocking and streaming requests
- Model management and validation

## Limitations

- Requires local Ollama installation
- Notes limited to 10K characters each
- No multi-user support (single-user CLI tool)
- Configuration stored in plaintext (including API keys)
- No offline mode - requires Supabase connection for note storage
- Limited to models available through Ollama
- No backup/restore functionality for notes

## Known Issues

- Setup process requires manual Supabase project creation
- Error handling could be more granular
- No automatic model downloading
- Limited test coverage
- Documentation references non-existent commands