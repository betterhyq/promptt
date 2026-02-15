//! Yes/no confirm prompt.

use crate::util::style;
use colour::{write_bold, write_gray};
use std::io::{self, BufRead, Write};

/// Confirm prompt options.
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

/// Runs confirm prompt. Returns true for yes, false for no.
pub fn run_confirm<R: BufRead, W: Write>(
    opts: &ConfirmPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<bool> {
    let mut buf = Vec::with_capacity(opts.message.len() + 32);
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let hint = if opts.initial {
        &opts.yes_option
    } else {
        &opts.no_option
    };
    let mut gray_buf = Vec::with_capacity(hint.len() + 16);
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
    let result_str: &str = if value { &opts.yes_msg } else { &opts.no_msg };
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
    fn confirm_prompt_options_default() {
        let opts = ConfirmPromptOptions::default();
        assert!(opts.message.is_empty());
        assert!(!opts.initial);
        assert_eq!(opts.yes_msg, "yes");
        assert_eq!(opts.no_msg, "no");
        assert_eq!(opts.yes_option, "(Y/n)");
        assert_eq!(opts.no_option, "(y/N)");
    }

    #[test]
    fn run_confirm_yes() {
        let opts = ConfirmPromptOptions {
            message: "Continue?".into(),
            initial: false,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"yes\n");
        let mut stdout = Vec::new();
        let r = run_confirm(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), true);
    }

    #[test]
    fn run_confirm_y_yes() {
        let opts = ConfirmPromptOptions {
            message: "Ok?".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"y\n");
        let mut stdout = Vec::new();
        assert_eq!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }

    #[test]
    fn run_confirm_no() {
        let opts = ConfirmPromptOptions {
            message: "Continue?".into(),
            initial: true,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"n\n");
        let mut stdout = Vec::new();
        let r = run_confirm(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), false);
    }

    #[test]
    fn run_confirm_empty_uses_initial() {
        let opts = ConfirmPromptOptions {
            message: "?".into(),
            initial: true,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        assert_eq!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }

    #[test]
    fn run_confirm_writes_yes_msg_when_true() {
        let opts = ConfirmPromptOptions {
            message: "Ok?".into(),
            initial: false,
            yes_msg: "confirmed".into(),
            no_msg: "cancelled".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"y\n");
        let mut stdout = Vec::new();
        assert!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap());
        let out = String::from_utf8(stdout).unwrap();
        assert!(out.contains("confirmed"));
    }

    #[test]
    fn run_confirm_no_returns_false() {
        let opts = ConfirmPromptOptions {
            message: "Ok?".into(),
            initial: true,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"no\n");
        let mut stdout = Vec::new();
        assert_eq!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }

    #[test]
    fn run_confirm_n_returns_false() {
        let opts = ConfirmPromptOptions {
            message: "Ok?".into(),
            initial: true,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"n\n");
        let mut stdout = Vec::new();
        assert_eq!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }

    #[test]
    fn run_confirm_empty_with_initial_false_returns_false() {
        let opts = ConfirmPromptOptions {
            message: "?".into(),
            initial: false,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        assert_eq!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap(), false);
    }

    #[test]
    fn run_confirm_uppercase_yes_accepted() {
        let opts = ConfirmPromptOptions {
            message: "?".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"YES\n");
        let mut stdout = Vec::new();
        assert_eq!(run_confirm(&opts, &mut stdin, &mut stdout).unwrap(), true);
    }
}
