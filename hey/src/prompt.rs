use std::io::{self, Write};

fn print_divider(width: usize) {
    println!("{}", "─".repeat(width));
}

/// Prints the prompt divider and `> ` marker, reads a line of input from
/// stdin, trims it, and returns it as the user's question. Returns an error
/// if the question is empty.
pub fn get_question(prompt_width: usize) -> Result<String, String> {
    print_divider(prompt_width);
    print!("> ");
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to write to stdout: {e}"))?;

    let mut question = String::new();
    io::stdin()
        .read_line(&mut question)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    let question = question.trim().to_string();
    print_divider(prompt_width);

    if question.is_empty() {
        return Err("No question provided.".to_string());
    }

    Ok(question)
}
