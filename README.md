# Logswise CLI

[![Lint & Format](https://github.com/k61b/logswise-cli/actions/workflows/lint.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/lint.yml)
[![Test](https://github.com/k61b/logswise-cli/actions/workflows/test.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Logswise CLI is a command-line tool for note-taking, context-aware suggestions, and AI chat, powered by your local LLM (Ollama) and Supabase. It is designed for developers and teams who want to capture notes, get actionable suggestions, and chat with an assistant—all from the terminal.

---

## 🚀 What's New?

- **Modular, DRY codebase:** Utility and service logic are now shared across features for clarity and maintainability.
- **Embedding-only mode:** If you configure an embedding model (see below), Logswise will run in fast semantic search mode—no LLM generation, just relevant notes.
- **Clear model configuration feedback:** The CLI tells you if you're using an embedding model or an LLM, and how to switch.
- **Context-rich prompts:** Both chat and suggestion features always use your user info and relevant notes for better results.
- **Improved error messages and troubleshooting tips.**

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

- **Take Notes:** Store your thoughts and ideas quickly from the CLI.
- **Get Suggestions:** Receive helpful, context-aware suggestions based on your queries, recent notes, and profile information. No suggestions table is required—everything is generated dynamically.
- **Chat with Assistant:** Engage in a conversation with an AI assistant, powered by your configured LLM (Ollama).
- **Semantic Search:** Use embedding-only mode for lightning-fast note retrieval.

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
- Supabase Project URL
- Supabase API Key

Your information is stored locally in `~/.logswise/setup.json`.

For detailed Supabase setup, see [SUPABASE_SETUP.md](SUPABASE_SETUP.md).

---

## Usage

After setup, you can use the following commands:

- **Take a Note:**
  ```sh
  logswise note "Your note here"
  ```
- **Get Suggestions:**
  ```sh
  logswise suggestion "What would you like suggestions for?"
  ```
- **Chat with Assistant:**
  ```sh
  logswise chat "Say anything you like"
  ```
- **Get Help:**
  ```sh
  logswise help
  ```

---

## How Suggestions & Chat Work

- Both features use your profile and recent notes as context for the LLM.
- If you use an embedding model, only semantic search is performed (embedding-only mode).
- If you use an LLM, you get context-rich suggestions and chat.
- The CLI will always tell you which mode is active and how to change it.

---

## Troubleshooting Model Configuration

- **If you see a message about "embedding-only mode":**
  - You are using an embedding model. Switch to an LLM for chat/suggestions.
- **If chat or suggestion commands do not generate text:**
  - Check your model name in `~/.logswise/setup.json`.
  - Make sure your Ollama server is running and the model is available.
  - Use an LLM for generation, or an embedding model for search only.
- **For best results:**
  - Use embedding models for fast search, LLMs for chat/suggestions.
  - Always keep your models up to date in Ollama.

---

## Project Structure

- `src/` — Rust source code
- `SUPABASE_SETUP.md` — Supabase setup instructions
- `LICENSE` — MIT License
- `CONTRIBUTING.md` — Contribution guidelines
- `CODE_OF_CONDUCT.md` — Code of conduct

---

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
