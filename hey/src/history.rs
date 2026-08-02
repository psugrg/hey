//! Persists conversation history per terminal, so `hey -f`/`--follow-up`
//! resumes the right conversation even when multiple terminals are open.
//!
//! Conversations are scoped by the controlling terminal device (e.g.
//! `/dev/pts/3` on Linux, `/dev/ttys003` on macOS), obtained via the POSIX
//! `ttyname()` call, so each terminal keeps its own history file.

use crate::client::ChatMessage;
use std::ffi::CStr;
use std::path::PathBuf;

unsafe extern "C" {
    fn ttyname(fd: i32) -> *const std::ffi::c_char;
}

/// Returns an identifier for the current controlling terminal, derived from
/// `ttyname(stdin)` and sanitized for use as a filename. Falls back to
/// `"default"` when stdin isn't attached to a terminal (e.g. piped input).
fn terminal_id() -> String {
    let ptr = unsafe { ttyname(0) };
    if ptr.is_null() {
        return "default".to_string();
    }

    let tty_path = unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();

    let sanitized: String = tty_path
        .trim_start_matches('/')
        .chars()
        .map(|c| if c == '/' { '-' } else { c })
        .collect();

    if sanitized.is_empty() {
        "default".to_string()
    } else {
        sanitized
    }
}

fn history_path() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME environment variable is not set.".to_string())?;
    Ok(PathBuf::from(home)
        .join(".cache")
        .join("hey")
        .join("history")
        .join(format!("{}.json", terminal_id())))
}

/// Loads the conversation history for the current terminal, if any exists.
/// Returns `None` if there's no history yet, or if it can't be read/parsed.
pub fn load() -> Option<Vec<ChatMessage>> {
    let path = history_path().ok()?;
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

/// Saves the conversation history for the current terminal.
pub fn save(messages: &[ChatMessage]) -> Result<(), String> {
    let path = history_path()?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create history directory: {e}"))?;
    }

    let contents = serde_json::to_string(messages)
        .map_err(|e| format!("Failed to serialize conversation history: {e}"))?;

    std::fs::write(path, contents).map_err(|e| format!("Failed to write conversation history: {e}"))
}
