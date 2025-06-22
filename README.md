# Logswise CLI

[![CI](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Logswise CLI is a command-line tool for note-taking, context-aware suggestions, and AI chat, powered by your local LLM (Ollama) and Supabase. It is designed for developers and teams who want to capture notes, get actionable suggestions, and chat with an assistant—all from the terminal.

---

## 🚀 What's New?

- **🔧 Health Check System:** New `doctor` command validates configuration and tests connectivity to Ollama/Supabase
- **⚡ Enhanced Error Handling:** Eliminated all dangerous `unwrap()` calls with graceful error recovery
- **✅ Input Validation:** Comprehensive validation for notes (10K character limit), queries, and messages
- **🛡️ Network Resilience:** 30-second timeouts and detailed error messages for connection failures
- **📋 Detailed CLI Help:** Enhanced help system with examples, troubleshooting guides, and getting started instructions
- **🎯 Command Improvements:** Added version support (`--version`), renamed help to `guide` to avoid conflicts
- **🔍 Configuration Validation:** URL format validation, API key checking, and model name validation
- **⚙️ Modular Architecture:** Clean separation of concerns with custom error types and validation framework
- **📊 Performance Framework:** Built-in timing utilities for operation monitoring

## 🚀 Usability Enhancements (Latest)

- **🏃 Command Shortcuts:** Use `n`, `s`, `c` as short forms for `note`, `suggestion`, `chat`
- **🔄 Interactive Mode:** Run `logswise-cli interactive` for a persistent session with guided menus
- **📊 Recent Notes:** View your latest notes with `logswise-cli recent --count 10`
- **🔧 Shell Completions:** Generate auto-completions for bash, zsh, fish, and PowerShell
- **💡 Smart Discovery:** Enhanced help with examples and common workflows

---

## 🤖 Embedding Models vs. LLMs

Logswise supports two types of models:

- **Embedding Models:** Used for fast semantic search (finding relevant notes). Examples:
  - `nomic-embed-text`
  - `bge-base-en`
  - `all-minilm`
- **LLMs (Large Language Models):** Used for generating suggestions and chat responses. Examples:
  - `llama3`
  - `deepseek-coder`
  - `mistral`
  - `phi3`

**How it works:**

- If you configure an embedding model, Logswise runs in "embedding-only mode": it finds and prints relevant notes, but does not generate new text.
- If you configure an LLM, Logswise uses your user info and relevant notes as context for suggestions and chat.

**Tip:**

- For chat and suggestions, use an LLM.
- For fast semantic search only, use an embedding model.

---

## ⚙️ Model Configuration

During `logswise-cli setup`, you will be asked for your LLM name. Enter the model name exactly as it appears in your Ollama server (or other LLM provider).

- **To use an LLM:** Enter a model like `llama3`, `deepseek-coder`, or `mistral`.
- **To use embedding-only mode:** Enter a model like `nomic-embed-text` or `bge-base-en`.

**You can change your model at any time** by editing `~/.logswise/setup.json` or re-running `logswise-cli setup`.

---

## 🧠 Embedding-Only Mode (Semantic Search)

If you configure an embedding model, Logswise will:

- Perform a fast semantic search for relevant notes.
- Print the most relevant notes to your query.
- **Not** generate new suggestions or chat responses.
- Show a message explaining that you are in embedding-only mode and how to switch to an LLM.

This is useful for quickly finding related notes without waiting for LLM generation.

---

## Features

- **📝 Take Notes:** Store your thoughts and ideas quickly from the CLI with input validation (up to 10,000 characters)
- **💡 Get Suggestions:** Receive helpful, context-aware suggestions based on your queries, recent notes, and profile information
- **🤖 Chat with Assistant:** Engage in conversation with an AI assistant, powered by your configured LLM (Ollama)
- **🔍 Semantic Search:** Use embedding-only mode for lightning-fast note retrieval
- **🔧 Health Diagnostics:** Comprehensive configuration and connectivity checking with the `doctor` command
- **⚡ Network Resilience:** 30-second timeouts and robust error handling for all network operations
- **📊 Progress Indicators:** Visual feedback during long-running operations
- **🛡️ Input Validation:** Comprehensive validation for all user inputs with helpful error messages
- **📋 Enhanced CLI:** Detailed help, examples, troubleshooting guides, and version information

---

## Installation

### Homebrew (Recommended)

```sh
brew tap k61b/tap
brew install logswise-cli
```

---

## Setup

Before using the application, you need to set up your profile and connect to your Supabase database and LLM.

```sh
logswise-cli setup
```

Follow the prompts to enter your profile information:

- Profession (e.g., Software Developer, Product Manager, Designer, Data Scientist, QA Engineer, DevOps Engineer, Sales Engineer, Technical Writer, Other)
- Job Title (e.g., Intern, Junior, Mid, Senior, Lead, Manager, Director, VP, C-level, Other)
- Company Name
- Company Size (e.g., 1-10, 10-100, 100-500, 500-1000, 1000-5000, 5000+)
- Years of Professional Experience (e.g., <1 year, 1-3 years, 3-5 years, 5-10 years, 10+ years)
- Preferred Programming Language (e.g., Rust, Python, JavaScript/TypeScript, Go, Java, C#, C/C++, Ruby, Swift, Kotlin, Other)
- Preferred Work Mode (Remote, On-site, Hybrid)
- **LLM Name (see above for options)**
- **Ollama Base URL** (default: http://localhost:11434)
- **Embedding Model** (default: nomic-embed-text)
- Supabase Project URL
- Supabase API Key

Your information is stored locally in `~/.logswise/setup.json`.

For detailed Supabase setup, see [SUPABASE_SETUP.md](SUPABASE_SETUP.md).

---

## Usage

After setup, you can use the following commands:

### Core Commands

- **Take a Note:**
  ```sh
  logswise-cli note "Your note here"
  # Or use the short form:
  logswise-cli n "Quick note"
  ```
- **Get Suggestions:**
  ```sh
  logswise-cli suggestion "What would you like suggestions for?"
  # Or use the short form:
  logswise-cli s "How to improve onboarding?"
  ```
- **Chat with Assistant:**
  ```sh
  logswise-cli chat "Say anything you like"
  # Or use the short form:
  logswise-cli c "Best practices for logging?"
  ```

### Enhanced Productivity Features

- **Interactive Mode (NEW!):**

  ```sh
  logswise-cli interactive
  ```

  Start a persistent session with guided menus for continuous note-taking and chatting.

- **Recent Notes (NEW!):**
  ```sh
  logswise-cli recent               # Show last 5 notes
  logswise-cli recent --count 10    # Show last 10 notes
  ```

### System Commands

- **Check Configuration Health:**

  ```sh
  logswise-cli doctor
  ```

  Run a comprehensive health check of your configuration and connectivity.

- **View Detailed Help:**

  ```sh
  logswise-cli guide
  ```

  Get enhanced help with examples, troubleshooting guides, and getting started instructions.

- **Show Version:**

  ```sh
  logswise-cli --version
  ```

  Display the current version of Logswise CLI.

- **Display Your Profile:**

  ```sh
  logswise-cli stats
  ```

  Show your profile information and configuration details.

- **Show About Information:**

  ```sh
  logswise-cli about
  ```

  Display information about Logswise CLI and its authors.

- **Generate Shell Completions (NEW!):**
  ```sh
  logswise-cli completions bash     # For Bash
  logswise-cli completions zsh      # For Zsh
  logswise-cli completions fish     # For Fish
  logswise-cli completions powershell # For PowerShell
  ```

### Quick Start Workflow

1. **First time setup:**

   ```sh
   logswise-cli setup
   ```

2. **Start working (choose your style):**

   **Option A: Interactive Mode (Recommended for new users)**

   ```sh
   logswise-cli interactive
   ```

   **Option B: Direct Commands (Great for power users)**

   ```sh
   logswise-cli n "Fixed critical bug in auth service"
   logswise-cli s "How to write better commit messages?"
   ```

3. **Review your work:**
   ```sh
   logswise-cli recent --count 5
   ```

### Additional Commands

- `logswise-cli how` - Explain how Logswise works
- `logswise-cli models` - Show information about embedding models vs LLMs
- `logswise-cli troubleshoot` - Show troubleshooting tips for model configuration
- `logswise-cli context` - Explain how context is used in suggestions and chat

---

## How Suggestions & Chat Work

- Both features use your profile and recent notes as context for the LLM.
- If you use an embedding model, only semantic search is performed (embedding-only mode).
- If you use an LLM, you get context-rich suggestions and chat.
- The CLI will always tell you which mode is active and how to change it.

---

## Troubleshooting & Health Check

### Doctor Command

The `doctor` command provides comprehensive health checking and diagnostics:

```sh
logswise-cli doctor
```

This will:

- ✅ Validate your configuration file
- ✅ Check URL formats and API key validity
- ✅ Test Ollama server connectivity
- ✅ Verify embedding model availability
- ✅ Test LLM model functionality
- ✅ Check Supabase connection
- 📊 Provide actionable recommendations for any issues found

### Model Configuration Issues

- **If you see a message about "embedding-only mode":**
  - You are using an embedding model. Switch to an LLM for chat/suggestions.
- **If chat or suggestion commands do not generate text:**
  - Run `logswise-cli doctor` to diagnose the issue
  - Check your model name in `~/.logswise/setup.json`
  - Make sure your Ollama server is running: `ollama serve`
  - Ensure the model is available: `ollama pull <model-name>`
  - Use an LLM for generation, or an embedding model for search only.
- **For connection issues:**
  - Verify Ollama is running on the correct URL
  - Check your Supabase URL and API key
  - Ensure network connectivity
- **For best results:**
  - Use embedding models for fast search, LLMs for chat/suggestions
  - Always keep your models up to date in Ollama
  - Run `logswise-cli doctor` after any configuration changes

---

## Project Structure

- `src/` — Rust source code
- `SUPABASE_SETUP.md` — Supabase setup instructions
- `LICENSE` — MIT License
- `CONTRIBUTING.md` — Contribution guidelines
- `CODE_OF_CONDUCT.md` — Code of conduct

---

## Quality & Reliability

Logswise CLI is built with production-grade quality standards:

- **🛡️ Zero Panic Code:** All dangerous `unwrap()` calls eliminated with proper error handling
- **✅ Comprehensive Testing:** 8/8 tests passing with continuous integration
- **🔍 Strict Linting:** Zero clippy warnings with `-D warnings` enforcement
- **⚡ Optimized Build:** 4.4MB release binary with full optimization
- **📊 Performance Monitoring:** Built-in timing framework for operation tracking
- **🔒 Input Validation:** Comprehensive validation for all user inputs
- **🌐 Network Resilience:** Timeout handling and graceful error recovery
- **📋 User Experience:** Detailed error messages and actionable diagnostics

## Linting & Formatting

This project uses [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) for code style and linting.

- To check formatting: `cargo fmt --all -- --check`
- To auto-format: `cargo fmt --all`
- To lint: `cargo clippy --all-targets --all-features -- -D warnings`

Linting and formatting are checked automatically on every pull request via GitHub Actions.

---

## Testing

To run all tests:

```sh
cargo test --all --verbose
```

Tests are run automatically on every pull request via GitHub Actions.

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## Code of Conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

---

## Security

See [SECURITY.md](SECURITY.md) for vulnerability reporting instructions.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
