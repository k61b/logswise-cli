# Logswise CLI

Logswise CLI is a command-line application designed to help users take notes, get suggestions, and chat with an assistant. This application is built using TypeScript and Rust, and connects to a Supabase database for storing notes. Suggestions and chat are powered by a local LLM (Ollama) using your profile and recent notes for context.

## Features

- **Take Notes**: Easily store your thoughts and ideas.
- **Get Suggestions**: Receive helpful, context-aware suggestions based on your queries, recent notes, and profile information. No suggestions table is required—everything is generated dynamically.
- **Chat with Assistant**: Engage in a conversation with an AI assistant, powered by your configured LLM (Ollama).

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/logswise-cli.git
   ```

2. Navigate to the project directory:
   ```
   cd logswise-cli
   ```

3. Install the dependencies (for TypeScript):
   ```
   npm install
   ```

4. (Optional) Build the Rust CLI:
   ```
   cargo build --release
   ```

## Setup

Before using the application, you need to set up your profile and connect to your Supabase database and LLM.

1. Run the setup command:
   ```
   npm run setup
   ```
   or, for the Rust CLI:
   ```
   ./target/release/logswise_cli_rs setup
   ```

2. Follow the prompts to enter your profile information:
   - Profession (a/b/c)
   - Job Title (a/b/c/d)
   - Company Name
   - Company Size (a/b/c/d/e)
   - LLM Name (e.g., llama3, deepseek-r1)
   - Ollama URL (default: http://localhost:11434/api/generate)
   - Supabase Project URL
   - Supabase API Key

This information will be stored securely in a local JSON file.

## Usage

After setup, you can use the following commands:

- **Take a Note**:
  ```
  lw note: "Your note here"
  ```

- **Get Suggestions** (context-aware, no DB table needed):
  ```
  lw suggestion: "What would you like suggestions for?"
  ```

- **Chat with Assistant**:
  ```
  lw chat: "Say anything you like"
  ```

- **Get Help**:
  ```
  lw help
  ```

## How Suggestions Work

Suggestions are generated dynamically using your profile and recent notes as context, and are powered by your local LLM (Ollama). There is no need for a `suggestions` table in your database. Make sure your Ollama server is running and the model you specify is available.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.