# Logswise CLI

[![CI](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)

A Rust-based command-line tool for note-taking with AI-powered suggestions and chat. Stores notes in Supabase and uses local Ollama models for AI features.

## Current Features

- **Note Storage**: Store text notes (up to 10K characters) in Supabase with vector embeddings
- **AI Suggestions**: Get contextual suggestions based on stored notes using local LLM
- **AI Chat**: Interactive chat with AI that has access to your note history
- **Interactive Mode**: Menu-driven interface for all operations
- **Personalization**: Configure AI behavior and response styles
- **Multiple Setup Modes**: Express, template-based, and full configuration options

## Technical Implementation

- **Database**: Supabase PostgreSQL with pgvector extension for semantic search
- **AI Backend**: Local Ollama models for text generation and embeddings
- **Configuration**: JSON-based config stored in `~/.logswise/setup.json`
- **Language**: Rust with async/await for concurrent operations
- **UI**: Terminal-based with colored output and interactive menus

## Installation

```sh
brew tap k61b/tap
brew install logswise-cli
```

## Quick Setup

1. **Install Ollama** (if you haven't already):
   ```sh
   curl -fsSL https://ollama.ai/install.sh | sh
   ollama serve
   ```

2. **Pull a model**:
   ```sh
   # For AI features (chat/suggestions):
   ollama pull llama3
   
   # Or for search-only:
   ollama pull nomic-embed-text
   ```

3. **Set up a Supabase project** at [supabase.com](https://supabase.com) (free tier works)

4. **Configure Logswise** (Enhanced Setup Options):
   ```sh
   # Quick setup - only 5 questions, smart defaults
   logswise-cli setup --express
   
   # Use a template/preset for your role
   logswise-cli setup --template
   
   # Traditional full setup
   logswise-cli setup
   
   # Update existing configuration selectively
   logswise-cli setup --update
   ```
   Follow the prompts to enter your info, Supabase URL/key, and model name.

   💡 **New**: Express setup gets you running in under 2 minutes!

5. **Initialize the database**:
   ```sh
   logswise-cli init
   ```
6. **Personalize your AI** (optional):
   ```sh
   logswise-cli personalize setup
   ```
   This allows the AI to learn from your notes and improve suggestions.


## Basic Usage

```sh
# Add a note
logswise-cli note "Fixed the login bug by updating the auth token validation"

# Get AI suggestions (uses your notes as context)
logswise-cli suggestion "How should I handle database migrations?"

# Chat with AI assistant
logswise-cli chat "What are the best practices for error handling?"

# View recent notes
logswise-cli recent --count 10
```

## Model Types

**Embedding Models** (search only):
- `nomic-embed-text`, `bge-base-en`, `all-minilm`
- Finds relevant notes but doesn't generate new text

**LLMs** (full AI features):  
- `llama3`, `deepseek-coder`, `mistral`, `phi3`
- Generates suggestions and chat responses using your notes as context

Change models anytime by editing `~/.logswise/setup.json` or re-running setup.

## Available Commands

The CLI has a simple command structure with these actual commands:

```sh
# Setup and configuration
logswise-cli setup                    # Full interactive setup
logswise-cli setup --express          # Quick setup with defaults
logswise-cli setup --template         # Choose from role-based templates
logswise-cli setup --update           # Update existing configuration
logswise-cli setup --import <file>    # Import configuration from file

# Database initialization
logswise-cli init                     # Create/verify database schema

# Interactive mode (default)
logswise-cli                          # Main interactive interface
logswise-cli simple                   # Same as above (explicit)

# Personalization
logswise-cli personalize setup        # Configure AI personalization
logswise-cli personalize update       # Update personalization settings
logswise-cli personalize show         # View current personalization
logswise-cli personalize feedback     # Provide feedback on suggestions

# System info
logswise-cli --version
```

**Note**: Commands like `note`, `suggestion`, `chat`, `recent`, `doctor`, and `stats` mentioned in some documentation are accessed through the interactive mode, not as direct CLI commands.

## Troubleshooting

**Common Issues:**

- **"embedding-only mode" message**: You're using an embedding model. Switch to an LLM for chat/suggestions.
- **Chat/suggestions don't work**: Run `logswise-cli doctor` to diagnose. Check that Ollama is running and your model is available.
- **Connection errors**: Verify Ollama URL and Supabase credentials in `~/.logswise/setup.json`

**Health check**: `logswise-cli doctor` validates your entire setup.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the [AGPL v3.0 License](LICENSE).
