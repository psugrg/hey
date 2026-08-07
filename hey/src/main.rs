use hey::client::{self, ChatMessage};
use hey::config::Config;
use hey::{cli, history, prompt, render};
use std::process::ExitCode;

fn run(buddy: Option<&str>, follow_up: bool) -> Result<(), String> {
    let config = Config::load(buddy)?;

    let question = prompt::get_question(&config.theme)?;

    let mut messages = if follow_up {
        history::load().unwrap_or_default()
    } else {
        Vec::new()
    };

    if messages.is_empty() {
        messages.push(ChatMessage::system(config.model.system_prompt.clone()));
    }
    messages.push(ChatMessage::user(question));

    let answer = client::fetch_answer_with_spinner(&config.model, &config.theme, &messages)?;

    messages.push(ChatMessage::assistant(answer.clone()));
    let _ = history::save(&messages);

    render::render_answer(&answer, &config.theme, config.prompt_width);

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let parsed = match cli::parse(&args) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Error: {err}");
            return ExitCode::FAILURE;
        }
    };

    if parsed.help {
        cli::print_help();
        return ExitCode::SUCCESS;
    }

    if parsed.version {
        println!("hey version {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    match run(parsed.buddy.as_deref(), parsed.follow_up) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
