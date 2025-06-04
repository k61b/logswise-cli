# Logswise CLI

Logswise CLI is a command-line tool for note-taking, context-aware suggestions, and AI chat, powered by your local LLM (Ollama) and Supabase. It is designed for developers and teams who want to capture notes, get actionable suggestions, and chat with an assistant—all from the terminal.

---

## Features

- **Take Notes:** Store your thoughts and ideas quickly from the CLI.
- **Get Suggestions:** Receive helpful, context-aware suggestions based on your queries, recent notes, and profile information. No suggestions table is required—everything is generated dynamically.
- **Chat with Assistant:** Engage in a conversation with an AI assistant, powered by your configured LLM (Ollama).

---

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/yourusername/logswise-cli.git
   cd logswise-cli
   ```
2. **Build the Rust CLI:**
   ```sh
   cargo build --release
   ```

---

## Setup

Before using the application, you need to set up your profile and connect to your Supabase database and LLM.

1. **Run the setup command:**
   ```sh
   ./target/release/logswise_cli_rs setup
   ```
2. **Follow the prompts to enter your profile information:**
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

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
