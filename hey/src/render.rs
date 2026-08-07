use crate::config::Theme;

/// Renders an answer, styling fenced code blocks (```lang ... ```) with a
/// top/bottom border (showing the language on the top border) and
/// printing the code lines themselves. Every line, including code block
/// borders, is nested inside a box using `theme.prompt_line` and
/// closed with `theme.prompt_close`.
pub fn render_answer(answer: &str, theme: &Theme, prompt_width: usize) {
    let mut in_code_block = false;
    let prompt_line = &theme.prompt_line;

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
                    "{prompt_line}{}──{label}{}{}",
                    theme.code_snippet_border_color,
                    "─".repeat(dash_count.max(2)),
                    theme.reset
                );
                in_code_block = true;
            } else {
                println!(
                    "{prompt_line}{}{}{}",
                    theme.code_snippet_border_color,
                    "─".repeat(prompt_width),
                    theme.reset
                );
                in_code_block = false;
            }
            continue;
        }

        if in_code_block {
            println!("{prompt_line}{}{line}{}", theme.code_snippet_text_color, theme.reset);
        } else {
            println!("{prompt_line}{line}");
        }
    }

    println!("{}", theme.prompt_close);
}
