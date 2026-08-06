use hey::config::{
    Config, DEFAULT_API_URL, DEFAULT_MODEL, DEFAULT_PROMPT_CLOSE_SYMBOL, DEFAULT_PROMPT_DONE_SYMBOL,
    DEFAULT_PROMPT_LINE_SYMBOL, DEFAULT_PROMPT_MARKER, DEFAULT_PROMPT_TOP_SYMBOL, DEFAULT_SPINNER_INTERVAL_MS,
    DEFAULT_SYSTEM_PROMPT,
};
use std::path::Path;
use std::sync::Mutex;

// `HOME` and `OPENROUTER_API_KEY` are process-global. Tests in this file run
// as separate threads within one process, so this mutex serializes access to
// prevent them from stepping on each other's env var changes.
static ENV_MUTEX: Mutex<()> = Mutex::new(());

fn with_env<T>(home: &Path, api_key: Option<&str>, f: impl FnOnce() -> T) -> T {
    let _guard = ENV_MUTEX.lock().unwrap();

    unsafe {
        std::env::set_var("HOME", home);
        match api_key {
            Some(key) => std::env::set_var("OPENROUTER_API_KEY", key),
            None => std::env::remove_var("OPENROUTER_API_KEY"),
        }
    }

    f()
}

fn write_hey_toml(home: &Path, contents: &str) {
    let config_dir = home.join(".config");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("hey.toml"), contents).unwrap();
}

#[test]
fn loads_defaults_when_hey_toml_is_missing() {
    let home = tempfile::tempdir().unwrap();

    let config = with_env(home.path(), Some("test-key"), Config::load).unwrap();

    assert_eq!(config.model.api_key, "test-key");
    assert_eq!(config.model.name, DEFAULT_MODEL);
    assert_eq!(config.model.api_url, DEFAULT_API_URL);
    assert_eq!(config.model.system_prompt, DEFAULT_SYSTEM_PROMPT);
    assert_eq!(config.theme.prompt_marker, DEFAULT_PROMPT_MARKER);
    assert_eq!(config.theme.prompt_top_symbol, DEFAULT_PROMPT_TOP_SYMBOL);
    assert_eq!(config.theme.prompt_line_symbol, DEFAULT_PROMPT_LINE_SYMBOL);
    assert_eq!(config.theme.prompt_done_symbol, DEFAULT_PROMPT_DONE_SYMBOL);
    assert_eq!(config.theme.prompt_close_symbol, DEFAULT_PROMPT_CLOSE_SYMBOL);
    assert_eq!(config.theme.spinner_interval_ms, DEFAULT_SPINNER_INTERVAL_MS);
}

#[test]
fn overrides_defaults_from_hey_toml() {
    let home = tempfile::tempdir().unwrap();
    write_hey_toml(
        home.path(),
        r#"
        model = "openai/gpt-4o"
        api_url = "https://example.com/api"
        system_prompt = "custom prompt"
        "#,
    );

    let config = with_env(home.path(), Some("test-key"), Config::load).unwrap();

    assert_eq!(config.model.name, "openai/gpt-4o");
    assert_eq!(config.model.api_url, "https://example.com/api");
    assert_eq!(config.model.system_prompt, "custom prompt");
}

#[test]
fn overrides_ui_symbols_from_hey_toml() {
    let home = tempfile::tempdir().unwrap();
    write_hey_toml(
        home.path(),
        r#"
        prompt_marker = "> "
        prompt_top_symbol = "◈"
        prompt_line_symbol = "│"
        prompt_done_symbol = "◇"
        prompt_close_symbol = "*"
        spinner_frames = ["|", "/", "-", "\\"]
        spinner_interval_ms = 200
        "#,
    );

    let config = with_env(home.path(), Some("test-key"), Config::load).unwrap();

    assert_eq!(config.theme.prompt_marker, "> ");
    assert_eq!(config.theme.prompt_top_symbol, "◈");
    assert_eq!(config.theme.prompt_line_symbol, "│");
    assert_eq!(config.theme.prompt_done_symbol, "◇");
    assert_eq!(config.theme.prompt_close_symbol, "*");
    assert_eq!(
        config.theme.spinner_frames,
        vec!["|".to_string(), "/".to_string(), "-".to_string(), "\\".to_string()]
    );
    assert_eq!(config.theme.spinner_interval_ms, 200);
}

#[test]
fn partial_overrides_fall_back_to_defaults() {
    let home = tempfile::tempdir().unwrap();
    write_hey_toml(home.path(), r#"model = "openai/gpt-4o""#);

    let config = with_env(home.path(), Some("test-key"), Config::load).unwrap();

    assert_eq!(config.model.name, "openai/gpt-4o");
    assert_eq!(config.model.api_url, DEFAULT_API_URL);
    assert_eq!(config.model.system_prompt, DEFAULT_SYSTEM_PROMPT);
    assert_eq!(config.theme.prompt_marker, DEFAULT_PROMPT_MARKER);
}

#[test]
fn partial_ui_symbol_overrides_fall_back_to_defaults() {
    let home = tempfile::tempdir().unwrap();
    write_hey_toml(home.path(), r#"prompt_marker = "> ""#);

    let config = with_env(home.path(), Some("test-key"), Config::load).unwrap();

    assert_eq!(config.theme.prompt_marker, "> ");
    assert_eq!(config.theme.prompt_top_symbol, DEFAULT_PROMPT_TOP_SYMBOL);
    assert_eq!(config.theme.prompt_line_symbol, DEFAULT_PROMPT_LINE_SYMBOL);
    assert_eq!(config.theme.prompt_done_symbol, DEFAULT_PROMPT_DONE_SYMBOL);
    assert_eq!(config.theme.prompt_close_symbol, DEFAULT_PROMPT_CLOSE_SYMBOL);
    assert_eq!(config.theme.spinner_interval_ms, DEFAULT_SPINNER_INTERVAL_MS);
}

#[test]
fn ignores_api_key_if_present_in_toml() {
    let home = tempfile::tempdir().unwrap();
    write_hey_toml(home.path(), r#"api_key = "should-be-ignored""#);

    let config = with_env(home.path(), Some("real-key"), Config::load).unwrap();

    assert_eq!(config.model.api_key, "real-key");
}

#[test]
fn fails_when_api_key_missing() {
    let home = tempfile::tempdir().unwrap();

    let result = with_env(home.path(), None, Config::load);

    assert!(result.is_err());
}

#[test]
fn fails_on_invalid_toml() {
    let home = tempfile::tempdir().unwrap();
    write_hey_toml(home.path(), "model = ");

    let result = with_env(home.path(), Some("test-key"), Config::load);

    assert!(result.is_err());
}
