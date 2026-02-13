//! Toggle prompt (mirrors prompts/lib/elements/toggle).

use crate::util::style;
use colour::write_bold;
use std::io::{self, BufRead, Write};

/// Options for a toggle prompt.
pub struct TogglePromptOptions {
    pub message: String,
    pub initial: bool,
    pub active: String,
    pub inactive: String,
}

impl Default for TogglePromptOptions {
    fn default() -> Self {
        Self {
            message: String::new(),
            initial: false,
            active: "on".into(),
            inactive: "off".into(),
        }
    }
}

/// Run a toggle prompt. Returns true for active, false for inactive.
pub fn run_toggle<R: BufRead, W: Write>(
    opts: &TogglePromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<bool> {
    let mut buf = Vec::new();
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let hint = if opts.initial {
        format!("(Y/n) {}", opts.active)
    } else {
        format!("(y/N) {}", opts.inactive)
    };
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    write!(stdout, "{} {} {} {}", symbol, msg, delim, hint)?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let raw = line.trim().to_lowercase();
    let value = if raw.is_empty() {
        opts.initial
    } else {
        raw == "y" || raw == "yes" || raw == "on"
    };
    let result_str = if value {
        opts.active.clone()
    } else {
        opts.inactive.clone()
    };
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(stdout, "\r{} {} {} {}", done_symbol, msg, done_delim, result_str)?;
    stdout.flush()?;
    Ok(value)
}
