pub fn print_help() {
    println!(
        "hey {version} - a simple command-line AI assistant

USAGE:
    hey [OPTIONS]

OPTIONS:
    -f, --follow-up      Continue the previous conversation with a follow-up question
    -V, --version        Print version information
    -h, --help           Print this help message

ENVIRONMENT:
    OPENROUTER_API_KEY   Required. Your OpenRouter.ai API key
    OPENROUTER_MODEL     Optional. Overrides the default AI model",
        version = env!("CARGO_PKG_VERSION")
    );
}
