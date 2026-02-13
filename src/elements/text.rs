//! Text prompt (mirrors prompts/lib/elements/text).

use crate::util::style::{self, InputStyle};
use colour::write_bold;
use std::io::{self, BufRead, Write};

/// Options for a text prompt.
pub struct TextPromptOptions {
    pub message: String,
    pub initial: Option<String>,
    pub style: InputStyle,
    pub error_msg: Option<String>,
}

impl Default for TextPromptOptions {
    fn default() -> Self {
        Self {
            message: String::new(),
            initial: None,
            style: InputStyle::Default,
            error_msg: Some("Please Enter A Valid Value".into()),
        }
    }
}

/// Run a text prompt. Returns the entered string (or initial if user submits empty).
pub fn run_text<R: BufRead, W: Write>(
    opts: &TextPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<String> {
    let (transform, _scale) = style::render_style(opts.style);
    let initial = opts.initial.as_deref().unwrap_or("");
    let mut output = Vec::new();
    write_bold!(&mut output, "{}", opts.message).ok();
    let msg_styled = String::from_utf8_lossy(&output).into_owned();
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    let placeholder = initial.to_string();
    let prompt_line = format!("{} {} {} {}", symbol, msg_styled, delim, placeholder);
    write!(stdout, "{}", prompt_line)?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let value = line.trim().to_string();
    let value = if value.is_empty() { initial.to_string() } else { value };
    let rendered = transform.render(&value, opts.style);
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(stdout, "\r{} {} {} {}", done_symbol, msg_styled, done_delim, rendered)?;
    stdout.flush()?;
    Ok(value)
}
