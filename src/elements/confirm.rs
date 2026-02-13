//! Confirm (yes/no) prompt (mirrors prompts/lib/elements/confirm).

use crate::util::style;
use colour::{write_bold, write_gray};
use std::io::{self, BufRead, Write};

/// Options for a confirm prompt.
pub struct ConfirmPromptOptions {
    pub message: String,
    pub initial: bool,
    pub yes_msg: String,
    pub no_msg: String,
    pub yes_option: String,
    pub no_option: String,
}

impl Default for ConfirmPromptOptions {
    fn default() -> Self {
        Self {
            message: String::new(),
            initial: false,
            yes_msg: "yes".into(),
            no_msg: "no".into(),
            yes_option: "(Y/n)".into(),
            no_option: "(y/N)".into(),
        }
    }
}

/// Run a confirm prompt. Returns true for yes, false for no.
pub fn run_confirm<R: BufRead, W: Write>(
    opts: &ConfirmPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<bool> {
    let mut buf = Vec::new();
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let hint = if opts.initial {
        opts.yes_option.clone()
    } else {
        opts.no_option.clone()
    };
    let mut gray_buf = Vec::new();
    write_gray!(&mut gray_buf, "{}", hint).ok();
    let hint_styled = String::from_utf8_lossy(&gray_buf).into_owned();
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    write!(stdout, "{} {} {} {}", symbol, msg, delim, hint_styled)?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let raw = line.trim().to_lowercase();
    let value = if raw.is_empty() {
        opts.initial
    } else {
        raw == "y" || raw == "yes"
    };
    let result_str = if value {
        opts.yes_msg.clone()
    } else {
        opts.no_msg.clone()
    };
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(stdout, "\r{} {} {} {}", done_symbol, msg, done_delim, result_str)?;
    stdout.flush()?;
    Ok(value)
}
