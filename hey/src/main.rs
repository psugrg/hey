mod config;

use config::{Config, Model, Theme};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn print_divider(width: usize) {
    println!("{}", "─".repeat(width));
}

/// Renders an answer, styling fenced code blocks (```lang ... ```) with a
/// top/bottom border (showing the language on the top border) and
/// printing the code lines themselves.
fn render_answer(answer: &str, theme: &Theme, prompt_width: usize) {
    let mut in_code_block = false;

    for line in answer.lines() {
        let trimmed = line.trim_start();

        if let Some(fence_rest) = trimmed.strip_prefix("```") {
            if !in_code_block {
                let lang = fence_rest.trim();
                let label = if lang.is_empty() {
                    String::new()
                } else {
                    format!(" {lang} ")
                };
                let dash_count = prompt_width.saturating_sub(label.chars().count() + 2);
                println!(
                    "{}──{label}{}{}",
                    theme.code_snippet_border_color,
                    "─".repeat(dash_count.max(2)),
                    theme.reset
                );
                in_code_block = true;
            } else {
                println!(
                    "{}{}{}",
                    theme.code_snippet_border_color,
                    "─".repeat(prompt_width),
                    theme.reset
                );
                in_code_block = false;
            }
            continue;
        }

        if in_code_block {
            println!("{}{line}{}", theme.code_snippet_text_color, theme.reset);
        } else {
            println!("{line}");
        }
    }
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
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

fn run() -> Result<(), String> {
    let config = Config::load()?;

    print_divider(config.prompt_width);
    print!("> ");
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to write to stdout: {e}"))?;

    let mut question = String::new();
    io::stdin()
        .read_line(&mut question)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    let question = question.trim();
    print_divider(config.prompt_width);

    if question.is_empty() {
        return Err("No question provided.".to_string());
    }

    let request_body = ChatRequest {
        model: config.model.name.clone(),
        messages: vec![
            ChatMessage {
                role: "system",
                content: config.model.system_prompt.clone(),
            },
            ChatMessage {
                role: "user",
                content: question.to_string(),
            },
        ],
    };

    let answer = fetch_answer_with_spinner(&config.model, request_body)?;

    render_answer(&answer, &config.theme, config.prompt_width);

    Ok(())
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

fn fetch_answer_with_spinner(model: &Model, request_body: ChatRequest) -> Result<String, String> {
    let (tx, rx) = mpsc::channel();

    let api_key = model.api_key.clone();
    let api_url = model.api_url.clone();

    thread::spawn(move || {
        let result = send_chat_request(&api_key, &api_url, &request_body);
        // Ignore send errors: if the receiver is gone there's nothing to do.
        let _ = tx.send(result);
    });

    let _cursor_guard = CursorGuard::new();

    let dot_counts = [1usize, 2, 3];
    let mut frame = 0;

    let result = loop {
        match rx.try_recv() {
            Ok(result) => break result,
            Err(mpsc::TryRecvError::Disconnected) => {
                break Err("Background request thread ended unexpectedly.".to_string());
            }
            Err(mpsc::TryRecvError::Empty) => {
                let dots = ".".repeat(dot_counts[frame % dot_counts.len()]);
                print!("\x1b[2K\rThinking{dots}");
                let _ = io::stdout().flush();
                frame += 1;
                thread::sleep(Duration::from_millis(400));
            }
        }
    };

    // Clear the "Thinking..." line before printing the final answer/error.
    print!("\x1b[2K\r");
    let _ = io::stdout().flush();

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

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        if arg == "--version" || arg == "-V" {
            println!("hey version {}", env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
    }

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
