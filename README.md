# Logswise CLI

[![CI](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A command-line tool for note-taking with AI-powered suggestions and chat. Stores your notes in Supabase and uses local Ollama models for AI features.

## What it does

- **Notes**: Store text notes in a Supabase database (up to 10K characters each)
- **AI Suggestions**: Get contextual suggestions based on your query and stored notes
- **AI Chat**: Chat with an AI assistant that knows about your notes

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

4. **Configure Logswise**:
   ```sh
   logswise-cli setup
   ```
   Follow the prompts to enter your info, Supabase URL/key, and model name.

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

## All Commands

**Shortcuts**: `n` = `note`, `s` = `suggestion`, `c` = `chat`

```sh
# Core commands
logswise-cli note "Your note content"
logswise-cli suggestion "What should I do about X?"
logswise-cli chat "Tell me about Y"
logswise-cli recent --count 10

# Interactive mode (menu-driven interface)
logswise-cli interactive

# Configuration and health
logswise-cli setup      # Initial setup
logswise-cli doctor     # Check configuration and connectivity
logswise-cli init       # Set up database tables

# Profile and personalization
logswise-cli personalize setup    # Configure AI personalization
logswise-cli personalize show     # View current settings
logswise-cli stats                 # Show your profile

# Utilities
logswise-cli --version
logswise-cli completions zsh      # Generate shell completions
```

## Troubleshooting

**Common Issues:**

- **"embedding-only mode" message**: You're using an embedding model. Switch to an LLM for chat/suggestions.
- **Chat/suggestions don't work**: Run `logswise-cli doctor` to diagnose. Check that Ollama is running and your model is available.
- **Connection errors**: Verify Ollama URL and Supabase credentials in `~/.logswise/setup.json`

**Health check**: `logswise-cli doctor` validates your entire setup.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.
