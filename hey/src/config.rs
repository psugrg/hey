//! Application configuration: AI model settings and UI theme.
//!
//! The API key always comes from the `OPENROUTER_API_KEY` environment
//! variable. The model, API URL, system prompt and UI symbols can be
//! overridden via `~/.config/hey.toml`; if that file is absent, or a value
//! is omitted, the hardcoded defaults below are used.

use serde::Deserialize;

/// Default model used when `hey.toml` doesn't set one.
pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
/// Default OpenRouter API URL used when `hey.toml` doesn't set one.
pub const DEFAULT_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
/// Default system prompt used when `hey.toml` doesn't set one.
pub const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful assistant that answers questions about command-line tools and commands (e.g. bash, ls, grep, cat, find, etc). Keep answers concise and focused on CLI usage.";
const DEFAULT_PROMPT_WIDTH: usize = 60;
/// Default question marker used when `hey.toml` doesn't set one.
pub const DEFAULT_PROMPT_MARKER: &str = "› ";
/// Default symbol printed above the question when `hey.toml` doesn't set one.
pub const DEFAULT_PROMPT_OPEN: &str = "●";
/// Default box line symbol used when `hey.toml` doesn't set one.
pub const DEFAULT_PROMPT_LINE: &str = "╎";
/// Default symbol shown once the answer is ready, replacing the spinner,
/// when `hey.toml` doesn't set one.
pub const DEFAULT_PROMPT_DONE: &str = "○";
/// Default symbol printed on its own line to close the answer box when
/// `hey.toml` doesn't set one.
pub const DEFAULT_PROMPT_CLOSE: &str = "◉";
/// Default spinner animation frames used when `hey.toml` doesn't set them.
pub const DEFAULT_SPINNER_FRAMES: &[&str] = &["◜", "◝", "◞", "◟"];
/// Default delay between spinner frames, in milliseconds, used when
/// `hey.toml` doesn't set one.
pub const DEFAULT_SPINNER_INTERVAL_MS: u64 = 120;

/// Settings needed to talk to the AI model API.
pub struct Model {
    pub api_key: String,
    pub name: String,
    pub api_url: String,
    pub system_prompt: String,
}

/// UI theme: styling and symbols used when rendering the prompt and the
/// answer, namely the code snippet border (top/bottom divider with
/// language label), the code snippet text itself, and the prompt/response
/// box symbols and spinner animation.
pub struct Theme {
    pub code_snippet_border_color: &'static str,
    pub code_snippet_text_color: &'static str,
    pub reset: &'static str,
    pub prompt_marker: String,
    pub prompt_open: String,
    pub prompt_line: String,
    pub prompt_done: String,
    pub prompt_close: String,
    pub spinner_frames: Vec<String>,
    pub spinner_interval_ms: u64,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            code_snippet_border_color: "\x1b[90m",
            code_snippet_text_color: "\x1b[94m",
            reset: "\x1b[0m",
            prompt_marker: DEFAULT_PROMPT_MARKER.to_string(),
            prompt_open: DEFAULT_PROMPT_OPEN.to_string(),
            prompt_line: DEFAULT_PROMPT_LINE.to_string(),
            prompt_done: DEFAULT_PROMPT_DONE.to_string(),
            prompt_close: DEFAULT_PROMPT_CLOSE.to_string(),
            spinner_frames: DEFAULT_SPINNER_FRAMES.iter().map(|s| s.to_string()).collect(),
            spinner_interval_ms: DEFAULT_SPINNER_INTERVAL_MS,
        }
    }
}

/// Application configuration.
pub struct Config {
    pub model: Model,
    pub theme: Theme,
    pub prompt_width: usize,
}

/// Shape of `~/.config/hey.toml`. All fields are optional; missing fields
/// fall back to the hardcoded defaults.
#[derive(Deserialize, Default, Debug, PartialEq)]
struct FileConfig {
    model: Option<String>,
    api_url: Option<String>,
    system_prompt: Option<String>,
    prompt_marker: Option<String>,
    prompt_open: Option<String>,
    prompt_line: Option<String>,
    prompt_done: Option<String>,
    prompt_close: Option<String>,
    spinner_frames: Option<Vec<String>>,
    spinner_interval_ms: Option<u64>,
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
fn build_model(api_key: String, file_config: &FileConfig) -> Model {
    Model {
        api_key,
        name: file_config.model.clone().unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        api_url: file_config.api_url.clone().unwrap_or_else(|| DEFAULT_API_URL.to_string()),
        system_prompt: file_config
            .system_prompt
            .clone()
            .unwrap_or_else(|| DEFAULT_SYSTEM_PROMPT.to_string()),
    }
}

/// Builds the [`Theme`] settings from the optional file overrides, falling
/// back to the hardcoded defaults for any field the file doesn't set.
fn build_theme(file_config: &FileConfig) -> Theme {
    let defaults = Theme::default();

    Theme {
        prompt_marker: file_config.prompt_marker.clone().unwrap_or(defaults.prompt_marker),
        prompt_open: file_config
            .prompt_open
            .clone()
            .unwrap_or(defaults.prompt_open),
        prompt_line: file_config
            .prompt_line
            .clone()
            .unwrap_or(defaults.prompt_line),
        prompt_done: file_config
            .prompt_done
            .clone()
            .unwrap_or(defaults.prompt_done),
        prompt_close: file_config
            .prompt_close
            .clone()
            .unwrap_or(defaults.prompt_close),
        spinner_frames: file_config.spinner_frames.clone().unwrap_or(defaults.spinner_frames),
        spinner_interval_ms: file_config.spinner_interval_ms.unwrap_or(defaults.spinner_interval_ms),
        ..defaults
    }
}

impl Config {
    /// Loads configuration from `OPENROUTER_API_KEY` (required) and
    /// `~/.config/hey.toml` (optional overrides for model, API URL, system
    /// prompt and UI symbols).
    pub fn load() -> Result<Self, String> {
        let api_key = std::env::var("OPENROUTER_API_KEY").map_err(|_| {
            "OPENROUTER_API_KEY environment variable is not set.\n\
             Set it with: export OPENROUTER_API_KEY=\"your-api-key-here\""
                .to_string()
        })?;

        let file_config = FileConfig::load()?;

        Ok(Config {
            model: build_model(api_key, &file_config),
            theme: build_theme(&file_config),
            prompt_width: DEFAULT_PROMPT_WIDTH,
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
            prompt_marker = "> "
            prompt_open = "◈"
            prompt_line = "│"
            prompt_done = "◇"
            prompt_close = "*"
            spinner_frames = ["|", "/", "-", "\\"]
            spinner_interval_ms = 200
            "#,
        )
        .unwrap();

        assert_eq!(file_config.model.as_deref(), Some("openai/gpt-4o"));
        assert_eq!(file_config.api_url.as_deref(), Some("https://example.com/api"));
        assert_eq!(file_config.system_prompt.as_deref(), Some("custom prompt"));
        assert_eq!(file_config.prompt_marker.as_deref(), Some("> "));
        assert_eq!(file_config.prompt_open.as_deref(), Some("◈"));
        assert_eq!(file_config.prompt_line.as_deref(), Some("│"));
        assert_eq!(file_config.prompt_done.as_deref(), Some("◇"));
        assert_eq!(file_config.prompt_close.as_deref(), Some("*"));
        assert_eq!(
            file_config.spinner_frames,
            Some(vec!["|".to_string(), "/".to_string(), "-".to_string(), "\\".to_string()])
        );
        assert_eq!(file_config.spinner_interval_ms, Some(200));
    }

    #[test]
    fn parses_partial_toml() {
        let file_config = FileConfig::parse(r#"model = "openai/gpt-4o""#).unwrap();

        assert_eq!(file_config.model.as_deref(), Some("openai/gpt-4o"));
        assert_eq!(file_config.api_url, None);
        assert_eq!(file_config.system_prompt, None);
        assert_eq!(file_config.prompt_marker, None);
        assert_eq!(file_config.spinner_frames, None);
        assert_eq!(file_config.spinner_interval_ms, None);
    }

    #[test]
    fn parses_empty_toml_as_defaults() {
        let file_config = FileConfig::parse("").unwrap();

        assert_eq!(file_config.model, None);
        assert_eq!(file_config.api_url, None);
        assert_eq!(file_config.system_prompt, None);
        assert_eq!(file_config.prompt_marker, None);
        assert_eq!(file_config.prompt_open, None);
        assert_eq!(file_config.prompt_line, None);
        assert_eq!(file_config.prompt_done, None);
        assert_eq!(file_config.prompt_close, None);
        assert_eq!(file_config.spinner_frames, None);
        assert_eq!(file_config.spinner_interval_ms, None);
    }

    #[test]
    fn rejects_invalid_toml() {
        let result = FileConfig::parse("model = ");

        assert!(result.is_err());
    }

    #[test]
    fn build_model_falls_back_to_defaults_when_file_config_is_empty() {
        let model = build_model("key".to_string(), &FileConfig::default());

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
            ..FileConfig::default()
        };

        let model = build_model("key".to_string(), &file_config);

        assert_eq!(model.api_key, "key");
        assert_eq!(model.name, "custom/model");
        assert_eq!(model.api_url, "https://custom.example/api");
        assert_eq!(model.system_prompt, "custom prompt");
    }

    #[test]
    fn build_model_mixes_overrides_and_defaults() {
        let file_config = FileConfig {
            model: Some("custom/model".to_string()),
            ..FileConfig::default()
        };

        let model = build_model("key".to_string(), &file_config);

        assert_eq!(model.name, "custom/model");
        assert_eq!(model.api_url, DEFAULT_API_URL);
        assert_eq!(model.system_prompt, DEFAULT_SYSTEM_PROMPT);
    }

    #[test]
    fn theme_default_has_expected_values() {
        let theme = Theme::default();

        assert_eq!(theme.code_snippet_border_color, "\x1b[90m");
        assert_eq!(theme.code_snippet_text_color, "\x1b[94m");
        assert_eq!(theme.reset, "\x1b[0m");
        assert_eq!(theme.prompt_marker, DEFAULT_PROMPT_MARKER);
        assert_eq!(theme.prompt_open, DEFAULT_PROMPT_OPEN);
        assert_eq!(theme.prompt_line, DEFAULT_PROMPT_LINE);
        assert_eq!(theme.prompt_done, DEFAULT_PROMPT_DONE);
        assert_eq!(theme.prompt_close, DEFAULT_PROMPT_CLOSE);
        assert_eq!(
            theme.spinner_frames,
            DEFAULT_SPINNER_FRAMES.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
        assert_eq!(theme.spinner_interval_ms, DEFAULT_SPINNER_INTERVAL_MS);
    }

    #[test]
    fn build_theme_falls_back_to_defaults_when_file_config_is_empty() {
        let theme = build_theme(&FileConfig::default());
        let defaults = Theme::default();

        assert_eq!(theme.prompt_marker, defaults.prompt_marker);
        assert_eq!(theme.prompt_open, defaults.prompt_open);
        assert_eq!(theme.prompt_line, defaults.prompt_line);
        assert_eq!(theme.prompt_done, defaults.prompt_done);
        assert_eq!(theme.prompt_close, defaults.prompt_close);
        assert_eq!(theme.spinner_frames, defaults.spinner_frames);
        assert_eq!(theme.spinner_interval_ms, defaults.spinner_interval_ms);
    }

    #[test]
    fn build_theme_uses_file_overrides_when_present() {
        let file_config = FileConfig {
            prompt_marker: Some("> ".to_string()),
            prompt_open: Some("◈".to_string()),
            prompt_line: Some("│".to_string()),
            prompt_done: Some("◇".to_string()),
            prompt_close: Some("*".to_string()),
            spinner_frames: Some(vec!["|".to_string(), "/".to_string()]),
            spinner_interval_ms: Some(250),
            ..FileConfig::default()
        };

        let theme = build_theme(&file_config);

        assert_eq!(theme.prompt_marker, "> ");
        assert_eq!(theme.prompt_open, "◈");
        assert_eq!(theme.prompt_line, "│");
        assert_eq!(theme.prompt_done, "◇");
        assert_eq!(theme.prompt_close, "*");
        assert_eq!(theme.spinner_frames, vec!["|".to_string(), "/".to_string()]);
        assert_eq!(theme.spinner_interval_ms, 250);
    }

    #[test]
    fn build_theme_mixes_overrides_and_defaults() {
        let file_config = FileConfig {
            prompt_marker: Some("> ".to_string()),
            ..FileConfig::default()
        };

        let theme = build_theme(&file_config);
        let defaults = Theme::default();

        assert_eq!(theme.prompt_marker, "> ");
        assert_eq!(theme.prompt_open, defaults.prompt_open);
        assert_eq!(theme.spinner_interval_ms, defaults.spinner_interval_ms);
    }
}
