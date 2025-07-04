pub struct HelpHandler {}

impl HelpHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_about(&self) {
        println!("\nLogswise CLI: Effortless notes, context-aware suggestions, and AI chat for developers.\n");
        println!("- Take notes, get suggestions, and chat with your local LLM (Ollama)\n- Powered by Rust, Supabase, and your own context\n- Open source, privacy-first, and team-ready\n");
        println!("GitHub: https://github.com/k61b/logswise-cli\n");
    }

    pub fn print_how(&self) {
        println!("\nHow Logswise Works:\n");
        println!("- Setup your profile and connect to your own Supabase and LLM (Ollama).");
        println!("- Take notes, get suggestions, and chat—all from the CLI.");
        println!("- Suggestions are generated dynamically using your profile and recent notes as context.");
        println!("- No personal data is stored in the cloud—only your notes are synced to your Supabase.");
        println!("- Perfect for performance reviews, self-improvement, and transparent team collaboration.\n");
    }

    pub fn print_models(&self) {
        println!("\nEmbedding Models vs. LLMs:\n");
        println!("- Embedding Models (e.g., nomic-embed-text, bge-base-en, all-minilm): Used for fast semantic search. Enables embedding-only mode—finds relevant notes, but does not generate new text.");
        println!("- LLMs (e.g., llama3, deepseek-coder, mistral, phi3): Used for generating suggestions and chat responses, always using your profile and relevant notes as context.\n");
        println!("How to choose: Use an LLM for chat/suggestions, or an embedding model for fast search only.");
        println!("Tip: The CLI will tell you which mode is active and how to switch models.\n");
    }

    #[allow(dead_code)]
    pub fn print_troubleshoot(&self) {
        println!("\nTroubleshooting Model Configuration:\n");
        println!("- If you see a message about embedding-only mode, you are using an embedding model. Switch to an LLM for chat/suggestions.");
        println!("- If chat or suggestion commands do not generate text, check your model name in ~/.logswise/setup.json and ensure your Ollama server is running with the correct model.");
        println!("- For best results: use embedding models for fast search, LLMs for chat/suggestions. You can change your model at any time.\n");
    }

    #[allow(dead_code)]
    pub fn print_context(&self) {
        println!("\nContext Matters:\n");
        println!("- Both suggestion and chat features always use your profile and the most relevant notes as context for the LLM. This ensures that all responses are tailored to your real work and experience.");
        println!("- In embedding-only mode, only semantic search is performed and relevant notes are shown.\n");
    }
}
