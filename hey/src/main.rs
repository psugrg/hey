use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";
const SYSTEM_PROMPT: &str = "You are a helpful assistant that answers questions about command-line tools and commands (e.g. bash, ls, grep, cat, find, etc). Keep answers concise and focused on CLI usage.";
const PROMPT_WIDTH: usize = 60;

fn print_divider() {
    println!("{}", "─".repeat(PROMPT_WIDTH));
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
    let api_key = std::env::var("OPENROUTER_API_KEY").map_err(|_| {
        "OPENROUTER_API_KEY environment variable is not set.\n\
         Set it with: export OPENROUTER_API_KEY=\"your-api-key-here\""
            .to_string()
    })?;

    let model = std::env::var("OPENROUTER_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    print_divider();
    print!("> ");
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to write to stdout: {e}"))?;

    let mut question = String::new();
    io::stdin()
        .read_line(&mut question)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    let question = question.trim();
    print_divider();

    if question.is_empty() {
        return Err("No question provided.".to_string());
    }

    let request_body = ChatRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system",
                content: SYSTEM_PROMPT.to_string(),
            },
            ChatMessage {
                role: "user",
                content: question.to_string(),
            },
        ],
    };

    let answer = fetch_answer_with_spinner(api_key, request_body)?;

    println!("{answer}");

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

fn fetch_answer_with_spinner(api_key: String, request_body: ChatRequest) -> Result<String, String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let result = send_chat_request(&api_key, &request_body);
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

fn send_chat_request(api_key: &str, request_body: &ChatRequest) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(API_URL)
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
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
