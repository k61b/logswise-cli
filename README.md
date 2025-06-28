# Logswise CLI

[![CI](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/k61b/logswise-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**World-Class AI Assistant with Enterprise-Grade Intelligence**

Logswise CLI is a revolutionary command-line tool featuring **advanced AI personalization** with query intent recognition, strategic response frameworks, and context-aware intelligence. Built with industry-leading best practices, it delivers professional-grade assistance with **zero hallucination** and **maximum relevance** - comparable to leading enterprise AI solutions like GPT-4, Claude, and Copilot.

**🎯 Built for developers who demand excellence.**

---

## 🌟 **LATEST: Enterprise AI Personalization System**

### **🚀 Industry-Leading Features**
- **🧠 Advanced Context Analysis:** Intelligent query intent recognition with urgency assessment
- **🎯 Strategic Project Portfolio:** Priority-based project categorization with team dynamics
- **📈 Goal-Driven Roadmaps:** Visual progress tracking with timeline intelligence
- **🔒 Zero Hallucination:** Enterprise-grade safeguards against fabricated information
- **� Professional Excellence:** Structured responses with success metrics and follow-up mechanisms
- **⚡ Smart Response Frameworks:** Adaptive formats for meeting prep, progress reporting, and problem-solving

### **🎪 Communication Style Adaptation**
- **Executive Summary Style:** Concise, bullet-pointed recommendations (150-250 words)
- **Comprehensive Analysis Style:** Detailed reasoning with implementation steps (300-500 words)
- **Collaborative Advisor Style:** Friendly, encouraging language with motivational elements
- **Enterprise Consultant Style:** Formal, structured communication focused on business impact

### **🎯 Advanced Query Intelligence**
Automatically detects and optimizes for:
- **Meeting Preparation:** Agenda prep, progress summaries, challenge analysis
- **Progress Reporting:** Accomplishments, metrics, process improvements
- **Problem Solving:** Root cause analysis, solution options, trade-off evaluation
- **Career Growth:** Strategic positioning, skill development, advancement planning

---

## 🚀 **Core Platform Features**

### **🛠️ Automatic Database Setup**
- **One-Command Setup:** `logswise-cli setup` automatically creates Supabase tables and schema
- **Database Initialization:** Standalone `init` command for database setup and verification
- **Health Check System:** Comprehensive `doctor` command with schema validation

### **⚡ Enhanced Developer Experience**
- **Command Shortcuts:** Use `n`, `s`, `c` for `note`, `suggestion`, `chat`
- **Interactive Mode:** Persistent session with guided menus via `logswise-cli interactive`
- **Recent Notes:** Quick access with `logswise-cli recent --count 10`
- **Shell Completions:** Auto-completions for bash, zsh, fish, and PowerShell

### **🔧 Enterprise-Grade Reliability**
- **Input Validation:** 10K character limits with comprehensive validation
- **Network Resilience:** 30-second timeouts with graceful error recovery
- **Configuration Validation:** URL format checking and API key validation
- **Modular Architecture:** Clean separation with custom error types

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
## **🎯 Enterprise AI Capabilities**

### **Advanced Personalization Engine**
- **📊 Rich User Profiling:** Professional identity, cognitive preferences, learning velocity analysis
- **🚀 Strategic Project Portfolio:** Priority-based categorization with team dynamics and pressure analysis
- **🎯 Goal-Driven Development:** Visual progress tracking with timeline intelligence and milestone management
- **🧠 Interaction Analytics:** Continuous learning from user feedback and engagement patterns

### **🔍 Intelligent Context Processing**
- **Query Intent Recognition:** Automatically detects meeting prep, progress reporting, problem-solving contexts
- **Urgency Assessment:** Smart priority classification (high/medium/normal) based on temporal indicators
- **Semantic Knowledge Base:** Advanced note ranking and relevance scoring for contextual responses
- **Anti-Hallucination Framework:** Enterprise-grade safeguards against fabricated information

### **💡 Professional Response Generation**
- **Structured Output Frameworks:** Specialized templates for different professional scenarios
- **Success Metrics Integration:** Every suggestion includes measurable outcomes and tracking mechanisms
- **Stakeholder Impact Analysis:** Considers team dynamics and organizational context in recommendations
- **Follow-up Accountability:** Built-in checkpoints and review mechanisms for continuous improvement

---

## Features

- **🧠 Enterprise AI Personalization:** World-class context analysis with professional excellence standards
- **📝 Smart Note-Taking:** Store insights with semantic embedding and intelligent categorization (up to 10K characters)
- **💡 Strategic Suggestions:** Context-aware recommendations with success metrics and implementation timelines
- **🤖 Intelligent Chat Assistant:** Conversation-aware AI powered by your configured LLM with project context
- **🔍 Advanced Semantic Search:** Lightning-fast note retrieval with relevance ranking
- **🚀 Automatic Database Setup:** One-command setup creates optimized Supabase schema automatically
- **🔧 Health Diagnostics:** Comprehensive system validation with connectivity and configuration checking
- **⚙️ Database Management:** Dedicated `init` command for schema setup and verification
- **⚡ Enterprise Reliability:** 30-second timeouts, graceful error recovery, and robust network handling
- **📊 Real-time Feedback:** Visual progress indicators and detailed status reporting
- **🛡️ Input Validation:** Comprehensive validation with helpful error messages and security safeguards
- **📋 Professional CLI:** Enhanced help system, examples, troubleshooting guides, and version management

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

**🚀 New: Automatic Database Setup**

The setup process now automatically creates the required Supabase tables and schema! After providing your Supabase URL and API key, the CLI will:

- ✅ Test your Supabase connection
- ✅ Automatically create the `notes` table
- ✅ Set up embedding columns for semantic search
- ✅ Create necessary indexes and functions
- ✅ Verify everything is working correctly

If automatic setup is not available for your Supabase instance, you'll get clear instructions for manual setup.

**Additional Commands:**

- **Initialize Database:** If you need to set up the database later or verify your setup:

  ```sh
  logswise-cli init
  ```

- **Health Check:** Verify your configuration and test all connections:
  ```sh
  logswise-cli doctor
  ```

For detailed manual Supabase setup (if needed), see [SUPABASE_SETUP.md](SUPABASE_SETUP.md).

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

- **Advanced AI Personalization:**
  ```sh
  logswise-cli personalize
  ```
  
  Set up enhanced AI personalization with detailed professional context, goal-driven development tracking, and communication style preferences for enterprise-grade suggestions.
  
  **Personalization subcommands:**
  - `logswise-cli personalize setup` - Run the full personalization setup
  - `logswise-cli personalize update` - Update existing personalization settings  
  - `logswise-cli personalize show` - Show current personalization settings
  - `logswise-cli personalize feedback` - Provide feedback on suggestions

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

- **Set Up Advanced Personalization:**

  ```sh
  logswise-cli personalize
  ```

  Configure enterprise-grade AI personalization with detailed professional context, goal-driven development tracking, and communication style preferences for strategic intelligence and context-aware suggestions.

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
   logswise-cli personalize  # Configure advanced AI personalization
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
- `logswise-cli personalize setup` - Configure advanced AI personalization with professional context
- `logswise-cli personalize show` - Display current personalization settings and preferences

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
- ✅ Check Supabase connection and database schema
- ✅ Verify database table structure and write permissions
- 🔧 Detect missing database setup and provide guidance
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
