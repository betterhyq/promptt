//! Select prompt (mirrors prompts/lib/elements/select).

use crate::util::figures::Figures;
use crate::util::style;
use colour::{write_bold, write_cyan, write_gray};
use std::io::{self, BufRead, Write};

/// A single choice.
#[derive(Clone)]
pub struct Choice {
    pub title: String,
    pub value: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl Choice {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            description: None,
            disabled: false,
        }
    }
}

/// Options for a select prompt.
pub struct SelectPromptOptions {
    pub message: String,
    pub choices: Vec<Choice>,
    pub initial: Option<usize>,
    pub hint: Option<String>,
}

/// Run a select prompt. Returns the value of the selected choice.
pub fn run_select<R: BufRead, W: Write>(
    opts: &SelectPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<String> {
    let fig = Figures::new();
    let mut buf = Vec::new();
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    let hint = opts.hint.as_deref().unwrap_or("Use arrow-keys or type number. Return to submit.");
    let mut gray_buf = Vec::new();
    write_gray!(&mut gray_buf, "{}", hint).ok();
    let hint_styled = String::from_utf8_lossy(&gray_buf).into_owned();
    writeln!(stdout, "{} {} {}", symbol, msg, delim)?;
    for (i, c) in opts.choices.iter().enumerate() {
        let prefix = if c.disabled { " " } else { fig.pointer_small };
        let mut line_buf = Vec::new();
        write_cyan!(&mut line_buf, " {} ", (i + 1)).ok();
        let num = String::from_utf8_lossy(&line_buf).into_owned();
        let line = format!("  {} {} {}", num, prefix, c.title);
        writeln!(stdout, "{}", line)?;
    }
    writeln!(stdout, "  {}", hint_styled)?;
    write!(stdout, "  Answer (number or name): ")?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let raw = line.trim();
    let idx = if let Ok(n) = raw.parse::<usize>() {
        if n >= 1 && n <= opts.choices.len() {
            Some(n - 1)
        } else {
            None
        }
    } else {
        opts.choices.iter().position(|c| c.title.eq_ignore_ascii_case(raw) || c.value.eq_ignore_ascii_case(raw))
    };
    let idx = idx.or(opts.initial).unwrap_or(0);
    let choice = opts.choices.get(idx).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid choice"))?;
    if choice.disabled {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "selected option is disabled"));
    }
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(stdout, "\r{} {} {} {}", done_symbol, msg, done_delim, choice.title)?;
    stdout.flush()?;
    Ok(choice.value.clone())
}
