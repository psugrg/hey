mod client;
mod config;
mod prompt;
mod render;

use config::Config;
use std::process::ExitCode;

fn run() -> Result<(), String> {
    let config = Config::load()?;

    let question = prompt::get_question(config.prompt_width)?;

    let answer = client::fetch_answer_with_spinner(&config.model, &question)?;

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
