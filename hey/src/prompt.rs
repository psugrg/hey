use crate::config::Theme;
use std::io::{self, Write};

/// Prints the prompt header and marker, reads a line of input from stdin,
/// trims it, and returns it as the user's question. Returns an error if the
/// question is empty.
pub fn get_question(theme: &Theme) -> Result<String, String> {
    println!("{}", theme.prompt_open);
    print!("{}{}", theme.prompt_line, theme.prompt_marker);
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to write to stdout: {e}"))?;

    let mut question = String::new();
    io::stdin()
        .read_line(&mut question)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    let question = question.trim().to_string();

    if question.is_empty() {
        return Err("No question provided.".to_string());
    }

    Ok(question)
}
