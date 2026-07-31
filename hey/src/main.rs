mod client;
mod config;
mod render;

use config::Config;
use std::io::{self, Write};
use std::process::ExitCode;

fn print_divider(width: usize) {
    println!("{}", "─".repeat(width));
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

    let answer = client::fetch_answer_with_spinner(&config.model, question)?;

    render::render_answer(&answer, &config.theme, config.prompt_width);

    Ok(())
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
