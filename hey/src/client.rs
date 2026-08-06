use crate::config::Model;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// A single turn in a conversation, sent to and received from the
/// OpenRouter chat completions API, and used to persist history.
#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: String) -> Self {
        ChatMessage {
            role: "system".to_string(),
            content,
        }
    }

    pub fn user(content: String) -> Self {
        ChatMessage {
            role: "user".to_string(),
            content,
        }
    }

    pub fn assistant(content: String) -> Self {
        ChatMessage {
            role: "assistant".to_string(),
            content,
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

/// Hides the terminal cursor on creation and restores it when dropped,
/// so the cursor is always shown again even if we return early.
struct CursorGuard;

impl CursorGuard {
    fn new() -> Self {
        print!("\x1b[?25l");
        let _ = io::stdout().flush();
        CursorGuard
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        print!("\x1b[?25h");
        let _ = io::stdout().flush();
    }
}

/// Sends `messages` to the configured model, showing an animated `◜◝◞◟`
/// spinner in place while waiting, replacing it with `○` once the
/// response arrives, and returns the trimmed answer text.
pub fn fetch_answer_with_spinner(model: &Model, messages: &[ChatMessage]) -> Result<String, String> {
    let request_body = ChatRequest {
        model: model.name.clone(),
        messages: messages.to_vec(),
    };

    let (tx, rx) = mpsc::channel();

    let api_key = model.api_key.clone();
    let api_url = model.api_url.clone();

    thread::spawn(move || {
        let result = send_chat_request(&api_key, &api_url, &request_body);
        // Ignore send errors: if the receiver is gone there's nothing to do.
        let _ = tx.send(result);
    });

    let _cursor_guard = CursorGuard::new();

    let spinner_frames = ['◜', '◝', '◞', '◟'];
    let mut frame = 0;

    let result = loop {
        match rx.try_recv() {
            Ok(result) => break result,
            Err(mpsc::TryRecvError::Disconnected) => {
                break Err("Background request thread ended unexpectedly.".to_string());
            }
            Err(mpsc::TryRecvError::Empty) => {
                print!("\x1b[2K\r{}", spinner_frames[frame % spinner_frames.len()]);
                let _ = io::stdout().flush();
                frame += 1;
                thread::sleep(Duration::from_millis(120));
            }
        }
    };

    // Replace the spinner with "○" once the answer/error is ready.
    println!("\x1b[2K\r○");

    result
}

fn send_chat_request(
    api_key: &str,
    api_url: &str,
    request_body: &ChatRequest,
) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(api_url)
        .bearer_auth(api_key)
        .json(request_body)
        .send()
        .map_err(|e| format!("Failed to reach OpenRouter API: {e}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response
            .text()
            .unwrap_or_else(|_| "<no response body>".to_string());
        return Err(format!("OpenRouter API returned an error ({status}): {body}"));
    }

    let chat_response: ChatResponse = response
        .json()
        .map_err(|e| format!("Failed to parse OpenRouter API response: {e}"))?;

    chat_response
        .choices
        .first()
        .map(|choice| choice.message.content.trim().to_string())
        .ok_or_else(|| "OpenRouter API response contained no answer.".to_string())
}
