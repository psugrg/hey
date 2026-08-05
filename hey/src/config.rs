//! Application configuration: AI model settings and UI theme.
//!
//! The API key always comes from the `OPENROUTER_API_KEY` environment
//! variable. The model, API URL and system prompt can be overridden via
//! `~/.config/hey.toml`; if that file is absent, or a value is omitted,
//! the hardcoded defaults below are used.

use serde::Deserialize;

/// Default model used when `hey.toml` doesn't set one.
pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
/// Default OpenRouter API URL used when `hey.toml` doesn't set one.
pub const DEFAULT_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
/// Default system prompt used when `hey.toml` doesn't set one.
pub const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful assistant that answers questions about command-line tools and commands (e.g. bash, ls, grep, cat, find, etc). Keep answers concise and focused on CLI usage.";
const DEFAULT_PROMPT_WIDTH: usize = 60;
const DEFAULT_PROMPT_MARKER: &str = "› ";

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
#[derive(Deserialize, Default, Debug, PartialEq)]
struct FileConfig {
    model: Option<String>,
    api_url: Option<String>,
    system_prompt: Option<String>,
}

impl FileConfig {
    /// Parses the contents of a `hey.toml` file.
    fn parse(contents: &str) -> Result<Self, String> {
        toml::from_str(contents).map_err(|err| format!("Invalid hey.toml: {err}"))
    }

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

        FileConfig::parse(&contents).map_err(|err| format!("Failed to parse {}: {err}", path.display()))
    }
}

/// Builds the [`Model`] settings from the required API key and the optional
/// file overrides, falling back to the hardcoded defaults for any field the
/// file doesn't set.
fn build_model(api_key: String, file_config: FileConfig) -> Model {
    Model {
        api_key,
        name: file_config.model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        api_url: file_config.api_url.unwrap_or_else(|| DEFAULT_API_URL.to_string()),
        system_prompt: file_config
            .system_prompt
            .unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string()),
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
            model: build_model(api_key, file_config),
            theme: Theme::default(),
            prompt_width: DEFAULT_PROMPT_WIDTH,
            prompt_marker: DEFAULT_PROMPT_MARKER.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_toml() {
        let file_config = FileConfig::parse(
            r#"
            model = "openai/gpt-4o"
            api_url = "https://example.com/api"
            system_prompt = "custom prompt"
            "#,
        )
        .unwrap();

        assert_eq!(file_config.model.as_deref(), Some("openai/gpt-4o"));
        assert_eq!(file_config.api_url.as_deref(), Some("https://example.com/api"));
        assert_eq!(file_config.system_prompt.as_deref(), Some("custom prompt"));
    }

    #[test]
    fn parses_partial_toml() {
        let file_config = FileConfig::parse(r#"model = "openai/gpt-4o""#).unwrap();

        assert_eq!(file_config.model.as_deref(), Some("openai/gpt-4o"));
        assert_eq!(file_config.api_url, None);
        assert_eq!(file_config.system_prompt, None);
    }

    #[test]
    fn parses_empty_toml_as_defaults() {
        let file_config = FileConfig::parse("").unwrap();

        assert_eq!(file_config.model, None);
        assert_eq!(file_config.api_url, None);
        assert_eq!(file_config.system_prompt, None);
    }

    #[test]
    fn rejects_invalid_toml() {
        let result = FileConfig::parse("model = ");

        assert!(result.is_err());
    }

    #[test]
    fn build_model_falls_back_to_defaults_when_file_config_is_empty() {
        let model = build_model("key".to_string(), FileConfig::default());

        assert_eq!(model.api_key, "key");
        assert_eq!(model.name, DEFAULT_MODEL);
        assert_eq!(model.api_url, DEFAULT_API_URL);
        assert_eq!(model.system_prompt, DEFAULT_SYSTEM_PROMPT);
    }

    #[test]
    fn build_model_uses_file_overrides_when_present() {
        let file_config = FileConfig {
            model: Some("custom/model".to_string()),
            api_url: Some("https://custom.example/api".to_string()),
            system_prompt: Some("custom prompt".to_string()),
        };

        let model = build_model("key".to_string(), file_config);

        assert_eq!(model.api_key, "key");
        assert_eq!(model.name, "custom/model");
        assert_eq!(model.api_url, "https://custom.example/api");
        assert_eq!(model.system_prompt, "custom prompt");
    }

    #[test]
    fn build_model_mixes_overrides_and_defaults() {
        let file_config = FileConfig {
            model: Some("custom/model".to_string()),
            api_url: None,
            system_prompt: None,
        };

        let model = build_model("key".to_string(), file_config);

        assert_eq!(model.name, "custom/model");
        assert_eq!(model.api_url, DEFAULT_API_URL);
        assert_eq!(model.system_prompt, DEFAULT_SYSTEM_PROMPT);
    }

    #[test]
    fn theme_default_has_expected_colors() {
        let theme = Theme::default();

        assert_eq!(theme.code_snippet_border_color, "\x1b[90m");
        assert_eq!(theme.code_snippet_text_color, "\x1b[94m");
        assert_eq!(theme.reset, "\x1b[0m");
    }
}
