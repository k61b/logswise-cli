# Logswise CLI

[![Lint & Format](https://github.com/k61b/logswise-cli/actions/workflows/lint.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/lint.yml)
[![Test](https://github.com/k61b/logswise-cli/actions/workflows/test.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Logswise CLI is a command-line tool for note-taking, context-aware suggestions, and AI chat, powered by your local LLM (Ollama) and Supabase. It is designed for developers and teams who want to capture notes, get actionable suggestions, and chat with an assistant—all from the terminal.

---

## Features

- **Take Notes:** Store your thoughts and ideas quickly from the CLI.
- **Get Suggestions:** Receive helpful, context-aware suggestions based on your queries, recent notes, and profile information. No suggestions table is required—everything is generated dynamically.
- **Chat with Assistant:** Engage in a conversation with an AI assistant, powered by your configured LLM (Ollama).

---

## Installation

### Homebrew (Recommended)

```sh
brew tap k61b/tap
brew install logswise-cli
```

### (Optional) Define an Alias
To use `lw` as a shortcut for `logswise-cli`, add this to your `~/.zshrc` (or `~/.bashrc`):

```sh
echo 'alias lw="logswise-cli"' >> ~/.zshrc
source ~/.zshrc
```

---

## Setup

Before using the application, you need to set up your profile and connect to your Supabase database and LLM.

```sh
logswise-cli setup
# or, if you defined the alias:
lw setup
```

Follow the prompts to enter your profile information:
- Profession (e.g., Software Developer, Product Manager)
- Job Title (e.g., Mid, Senior, Lead, Manager)
- Company Name
- Company Size (e.g., 1-10, 10-100, 100-500, 500-1000, 1000+)
- LLM Name (e.g., llama3, deepseek-r1)
- Supabase Project URL
- Supabase API Key

Your information is stored locally in `~/.logswise/setup.json`.

For detailed Supabase setup, see [SUPABASE_SETUP.md](SUPABASE_SETUP.md).

---

## Usage

After setup, you can use the following commands:

- **Take a Note:**
  ```sh
  lw note "Your note here"
  ```
- **Get Suggestions:**
  ```sh
  lw suggestion "What would you like suggestions for?"
  ```
- **Chat with Assistant:**
  ```sh
  lw chat "Say anything you like"
  ```
- **Get Help:**
  ```sh
  lw help
  ```

---

## How Suggestions Work

Suggestions are generated dynamically using your profile and recent notes as context, powered by your local LLM (Ollama). There is no need for a `suggestions` table in your database. Make sure your Ollama server is running and the model you specify is available.

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
