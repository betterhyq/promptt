//! Select prompt.

use crate::util::figures::Figures;
use crate::util::style;
use colour::{write_bold, write_cyan, write_gray};
use std::io::{self, BufRead, Write};

/// Single choice option.
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

/// Select prompt options.
pub struct SelectPromptOptions {
    pub message: String,
    pub choices: Vec<Choice>,
    pub initial: Option<usize>,
    pub hint: Option<String>,
}

/// Runs select prompt. Returns value of selected choice.
pub fn run_select<R: BufRead, W: Write>(
    opts: &SelectPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<String> {
    let fig = Figures::default();
    let mut buf = Vec::with_capacity(opts.message.len() + 32);
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    let hint = opts
        .hint
        .as_deref()
        .unwrap_or("Use arrow-keys or type number. Return to submit.");
    let mut gray_buf = Vec::with_capacity(hint.len() + 16);
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
        if (1..=opts.choices.len()).contains(&n) {
            Some(n - 1)
        } else {
            None
        }
    } else {
        opts.choices
            .iter()
            .position(|c| c.title.eq_ignore_ascii_case(raw) || c.value.eq_ignore_ascii_case(raw))
    };
    let idx = idx.or(opts.initial).unwrap_or(0);
    let choice = opts
        .choices
        .get(idx)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid choice"))?;
    if choice.disabled {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "selected option is disabled",
        ));
    }
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(
        stdout,
        "\r{} {} {} {}",
        done_symbol, msg, done_delim, choice.title
    )?;
    stdout.flush()?;
    Ok(choice.value.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn choice_new() {
        let c = Choice::new("Title", "value");
        assert_eq!(c.title, "Title");
        assert_eq!(c.value, "value");
        assert!(c.description.is_none());
        assert!(!c.disabled);
    }

    #[test]
    fn run_select_by_number() {
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![
                Choice::new("One", "1"),
                Choice::new("Two", "2"),
                Choice::new("Three", "3"),
            ],
            initial: None,
            hint: None,
        };
        let mut stdin = Cursor::new(b"2\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "2");
    }

    #[test]
    fn run_select_by_title_case_insensitive() {
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![
                Choice::new("Apple", "apple"),
                Choice::new("Banana", "banana"),
            ],
            initial: None,
            hint: None,
        };
        let mut stdin = Cursor::new(b"BANANA\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "banana");
    }

    #[test]
    fn run_select_invalid_number_falls_back_to_initial_or_zero() {
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![Choice::new("A", "a"), Choice::new("B", "b")],
            initial: Some(1),
            hint: None,
        };
        let mut stdin = Cursor::new(b"xyz\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "b");
    }

    #[test]
    fn run_select_disabled_choice_returns_err() {
        let mut c = Choice::new("Disabled", "d");
        c.disabled = true;
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![Choice::new("A", "a"), c],
            initial: Some(1),
            hint: None,
        };
        let mut stdin = Cursor::new(b"2\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_err());
    }
}
