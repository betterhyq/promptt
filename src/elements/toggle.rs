//! Toggle prompt.

use crate::util::style;
use colour::write_bold;
use std::io::{self, BufRead, Write};

/// Toggle prompt options.
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

/// Runs toggle prompt. Returns true for active, false for inactive.
pub fn run_toggle<R: BufRead, W: Write>(
    opts: &TogglePromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<bool> {
    let mut buf = Vec::with_capacity(opts.message.len() + 32);
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let hint = if opts.initial {
        "(Y/n)"
    } else {
        "(y/N)"
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
    let result_str: &str = if value { &opts.active } else { &opts.inactive };
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(
        stdout,
        "\r{} {} {} {}",
        done_symbol, msg, done_delim, result_str
    )?;
    stdout.flush()?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn toggle_prompt_options_default() {
        let opts = TogglePromptOptions::default();
        assert!(opts.message.is_empty());
        assert!(!opts.initial);
        assert_eq!(opts.active, "on");
        assert_eq!(opts.inactive, "off");
    }

    #[test]
    fn run_toggle_on() {
        let opts = TogglePromptOptions {
            message: "Enable?".into(),
            initial: false,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"y\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }

    #[test]
    fn run_toggle_yes_on() {
        let opts = TogglePromptOptions {
            message: "?".into(),
            initial: false,
            active: "yes".into(),
            inactive: "no".into(),
        };
        let mut stdin = Cursor::new(b"yes\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }

    #[test]
    fn run_toggle_off() {
        let opts = TogglePromptOptions {
            message: "Enable?".into(),
            initial: true,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"n\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }

    #[test]
    fn run_toggle_empty_uses_initial() {
        let opts = TogglePromptOptions {
            message: "?".into(),
            initial: true,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }

    #[test]
    fn run_toggle_on_input_turns_on() {
        let opts = TogglePromptOptions {
            message: "Enable?".into(),
            initial: false,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"on\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }

    #[test]
    fn run_toggle_no_turns_off() {
        let opts = TogglePromptOptions {
            message: "Enable?".into(),
            initial: true,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"no\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }

    #[test]
    fn run_toggle_unknown_input_returns_false() {
        let opts = TogglePromptOptions {
            message: "?".into(),
            initial: true,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"maybe\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }

    #[test]
    fn run_toggle_unknown_input_with_initial_false_returns_false() {
        let opts = TogglePromptOptions {
            message: "?".into(),
            initial: false,
            active: "on".into(),
            inactive: "off".into(),
        };
        let mut stdin = Cursor::new(b"maybe\n");
        let mut stdout = Vec::new();
        assert_eq!(run_toggle(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }
}
