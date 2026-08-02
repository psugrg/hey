mod cli;
mod client;
mod config;
mod history;
mod prompt;
mod render;

use client::ChatMessage;
use config::Config;
use std::process::ExitCode;

fn run(follow_up: bool) -> Result<(), String> {
    let config = Config::load()?;

    let question = prompt::get_question(config.prompt_width, &config.prompt_marker)?;

    let mut messages = if follow_up {
        history::load().unwrap_or_default()
    } else {
        Vec::new()
    };

    if messages.is_empty() {
        messages.push(ChatMessage::system(config.model.system_prompt.clone()));
    }
    messages.push(ChatMessage::user(question));

    let answer = client::fetch_answer_with_spinner(&config.model, &messages)?;

    messages.push(ChatMessage::assistant(answer.clone()));
    let _ = history::save(&messages);

    render::render_answer(&answer, &config.theme, config.prompt_width);

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        cli::print_help();
        return ExitCode::SUCCESS;
    }

    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("hey version {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    let follow_up = args.iter().any(|arg| arg == "--follow-up" || arg == "-f");

    match run(follow_up) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
