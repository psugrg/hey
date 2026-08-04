//! Application configuration: AI model settings and UI theme.
//!
//! The API key always comes from the `OPENROUTER_API_KEY` environment
//! variable. The model, API URL and system prompt can be overridden via
//! `~/.config/hey.toml`; if that file is absent, or a value is omitted,
//! the hardcoded defaults below are used.

use serde::Deserialize;

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

/// Shape of `~/.config/hey.toml`. All fields are optional; missing fields
/// fall back to the hardcoded defaults.
#[derive(Deserialize, Default)]
struct FileConfig {
    model: Option<String>,
    api_url: Option<String>,
    system_prompt: Option<String>,
}

impl FileConfig {
    /// Reads and parses `~/.config/hey.toml`.
    ///
    /// Returns the default (empty) config if the file doesn't exist. Fails
    /// if the file exists but isn't valid TOML.
    fn load() -> Result<Self, String> {
        let path = match std::env::var("HOME") {
            Ok(home) => std::path::PathBuf::from(home).join(".config").join("hey.toml"),
            Err(_) => return Ok(FileConfig::default()),
        };

        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(_) => return Ok(FileConfig::default()),
        };

        toml::from_str(&contents)
            .map_err(|err| format!("Failed to parse {}: {err}", path.display()))
    }
}

impl Config {
    /// Loads configuration from `OPENROUTER_API_KEY` (required) and
    /// `~/.config/hey.toml` (optional overrides for model, API URL and
    /// system prompt).
    pub fn load() -> Result<Self, String> {
        let api_key = std::env::var("OPENROUTER_API_KEY").map_err(|_| {
            "OPENROUTER_API_KEY environment variable is not set.\n\
             Set it with: export OPENROUTER_API_KEY=\"your-api-key-here\""
                .to_string()
        })?;

        let file_config = FileConfig::load()?;

        Ok(Config {
            model: Model {
                api_key,
                name: file_config.model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
                api_url: file_config.api_url.unwrap_or_else(|| DEFAULT_API_URL.to_string()),
                system_prompt: file_config
                    .system_prompt
                    .unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string()),
            },
            theme: Theme::default(),
            prompt_width: DEFAULT_PROMPT_WIDTH,
            prompt_marker: DEFAULT_PROMPT_MARKER.to_string(),
        })
    }
}
