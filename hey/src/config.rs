//! Application configuration: AI model settings and UI theme.
//!
//! Currently sourced entirely from environment variables (plus fixed
//! defaults for values that aren't yet user-configurable). This module is
//! intentionally small but structured so it can grow (e.g. reading from a
//! config file) without changing how the rest of the app consumes it.

const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
const DEFAULT_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful assistant that answers questions about command-line tools and commands (e.g. bash, ls, grep, cat, find, etc). Keep answers concise and focused on CLI usage.";
const DEFAULT_PROMPT_WIDTH: usize = 60;
const DEFAULT_PROMPT_MARKER: &str = "> ";

/// Settings needed to talk to the AI model API.
pub struct Model {
    pub api_key: String,
    pub name: String,
    pub api_url: String,
    pub system_prompt: String,
}

/// UI theme: styling for the render elements used when displaying answers,
/// namely the code snippet border (top/bottom divider with language label)
/// and the code snippet text itself.
pub struct Theme {
    pub code_snippet_border_color: &'static str,
    pub code_snippet_text_color: &'static str,
    pub reset: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            code_snippet_border_color: "\x1b[90m",
            code_snippet_text_color: "\x1b[94m",
            reset: "\x1b[0m",
        }
    }
}

/// Application configuration.
pub struct Config {
    pub model: Model,
    pub theme: Theme,
    pub prompt_width: usize,
    pub prompt_marker: String,
}

impl Config {
    /// Loads configuration from the environment.
    ///
    /// Requires `OPENROUTER_API_KEY` to be set. `OPENROUTER_MODEL` is
    /// optional and defaults to [`DEFAULT_MODEL`].
    pub fn load() -> Result<Self, String> {
        let api_key = std::env::var("OPENROUTER_API_KEY").map_err(|_| {
            "OPENROUTER_API_KEY environment variable is not set.\n\
             Set it with: export OPENROUTER_API_KEY=\"your-api-key-here\""
                .to_string()
        })?;

        let name = std::env::var("OPENROUTER_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

        Ok(Config {
            model: Model {
                api_key,
                name,
                api_url: DEFAULT_API_URL.to_string(),
                system_prompt: DEFAULT_SYSTEM_PROMPT.to_string(),
            },
            theme: Theme::default(),
            prompt_width: DEFAULT_PROMPT_WIDTH,
            prompt_marker: DEFAULT_PROMPT_MARKER.to_string(),
        })
    }
}
