//! Select prompt.

use crate::util::figures::Figures;
use crate::util::style;
use ansi_escapes::EraseLines;
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
    /// Builds a choice with the given title and value; description and disabled are defaulted.
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

/// Read a single byte from a BufRead. Blocks until one is available.
pub(crate) fn read_byte<R: BufRead>(r: &mut R) -> io::Result<u8> {
    let buf = r.fill_buf()?;
    if buf.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected end of input",
        ));
    }
    let b = buf[0];
    r.consume(1);
    Ok(b)
}

/// Returns next enabled index when moving down, or same if none.
pub(crate) fn next_enabled(choices: &[Choice], current: usize) -> usize {
    for i in (current + 1)..choices.len() {
        if !choices[i].disabled {
            return i;
        }
    }
    current
}

/// Returns previous enabled index when moving up, or same if none.
pub(crate) fn prev_enabled(choices: &[Choice], current: usize) -> usize {
    for i in (0..current).rev() {
        if !choices[i].disabled {
            return i;
        }
    }
    current
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

    let mut selected = opts.initial.unwrap_or(0);
    if selected >= opts.choices.len() {
        selected = 0;
    }
    while opts.choices.get(selected).map(|c| c.disabled).unwrap_or(true) {
        let next = next_enabled(&opts.choices, selected);
        if next == selected {
            break;
        }
        selected = next;
    }

    let n_lines = opts.choices.len() + 3; // message, choices, hint, answer line

    let run_interactive = std::io::IsTerminal::is_terminal(&std::io::stdin());

    if run_interactive {
        let _guard = crossterm::terminal::enable_raw_mode()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let result = run_select_interactive(
            opts,
            stdin,
            stdout,
            &fig,
            &msg,
            &symbol,
            &delim,
            &hint_styled,
            &mut selected,
            n_lines,
        );

        crossterm::terminal::disable_raw_mode()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if let Err(e) = result {
            return Err(e);
        }
    } else {
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
        let raw = strip_arrow_escapes(line.trim());
        selected = parse_selection(opts, &raw)?;
    }

    let choice = opts
        .choices
        .get(selected)
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

/// Strip ANSI arrow key sequences from input so "5^[[B^[[A" becomes "5".
pub(crate) fn strip_arrow_escapes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        if b == 0x1b {
            if let (Some(0x5b), Some(c)) = (bytes.next(), bytes.next()) {
                if c == b'A' || c == b'B' || c == b'C' || c == b'D' {
                    continue;
                }
                out.push(0x1b as char);
                out.push(0x5b as char);
                out.push(c as char);
            } else {
                out.push(0x1b as char);
            }
        } else {
            out.push(b as char);
        }
    }
    out
}

/// Parse "number" or "name" into choice index.
pub(crate) fn parse_selection(opts: &SelectPromptOptions, raw: &str) -> io::Result<usize> {
    let idx = if let Ok(n) = raw.parse::<usize>() {
        if (1..=opts.choices.len()).contains(&n) {
            Some(n - 1)
        } else {
            None
        }
    } else {
        opts.choices.iter().position(|c| {
            c.title.eq_ignore_ascii_case(raw) || c.value.eq_ignore_ascii_case(raw)
        })
    };
    Ok(idx.or(opts.initial).unwrap_or(0))
}

fn run_select_interactive<R: BufRead, W: Write>(
    opts: &SelectPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
    fig: &Figures,
    msg: &str,
    symbol: &str,
    delim: &str,
    hint_styled: &str,
    selected: &mut usize,
    n_lines: usize,
) -> io::Result<()> {
    // In raw mode, \n alone does not move to column 0; use \r\n so each line starts at column 0.
    const NL: &str = "\r\n";

    fn write_choices(
        opts: &SelectPromptOptions,
        fig: &Figures,
        selected: usize,
        stdout: &mut dyn Write,
        nl: &str,
    ) -> io::Result<()> {
        for (i, c) in opts.choices.iter().enumerate() {
            let prefix = if c.disabled {
                " "
            } else if i == selected {
                fig.pointer_small
            } else {
                " "
            };
            let mut line_buf = Vec::new();
            write_cyan!(&mut line_buf, " {} ", (i + 1)).ok();
            let num = String::from_utf8_lossy(&line_buf).into_owned();
            write!(stdout, "  {} {} {}{}", num, prefix, c.title, nl)?;
        }
        Ok(())
    }

    write!(stdout, "{} {} {}{}", symbol, msg, delim, NL)?;
    write_choices(opts, fig, *selected, stdout, NL)?;
    write!(stdout, "  {}{}", hint_styled, NL)?;
    write!(stdout, "  Answer (number or name): ")?;
    stdout.flush()?;

    let mut typed = String::new();

    loop {
        let b = read_byte(stdin)?;
        if b == 0x0d || b == 0x0a {
            if !typed.is_empty() {
                let raw = strip_arrow_escapes(typed.trim());
                if let Ok(idx) = parse_selection(opts, &raw) {
                    if idx < opts.choices.len() && !opts.choices[idx].disabled {
                        *selected = idx;
                    }
                }
            }
            break;
        }
        if b == 0x1b {
            let b2 = read_byte(stdin).unwrap_or(0);
            let b3 = read_byte(stdin).unwrap_or(0);
            if b2 == 0x5b {
                match b3 {
                    b'A' => {
                        *selected = prev_enabled(&opts.choices, *selected);
                    }
                    b'B' => {
                        *selected = next_enabled(&opts.choices, *selected);
                    }
                    _ => {}
                }
            }
            // EraseLines erases the current line then moves up. We are on the "Answer" line,
            // so erase upward to clear the whole block. Cursor ends at top line, column 0.
            let up = n_lines as u16;
            write!(stdout, "{}", EraseLines(up))?;
            write!(stdout, "{} {} {}{}", symbol, msg, delim, NL)?;
            write_choices(opts, fig, *selected, stdout, NL)?;
            write!(stdout, "  {}{}", hint_styled, NL)?;
            write!(stdout, "  Answer (number or name): ")?;
            stdout.flush()?;
        } else if b.is_ascii_graphic() || b == b' ' {
            typed.push(b as char);
        }
    }
    Ok(())
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

    #[test]
    fn run_select_by_value() {
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![
                Choice::new("One", "val1"),
                Choice::new("Two", "val2"),
            ],
            initial: None,
            hint: None,
        };
        let mut stdin = Cursor::new(b"val2\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "val2");
    }

    #[test]
    fn run_select_with_custom_hint() {
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![Choice::new("A", "a")],
            initial: None,
            hint: Some("Custom hint".into()),
        };
        let mut stdin = Cursor::new(b"1\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        let out = String::from_utf8(stdout).unwrap();
        assert!(out.contains("Custom hint"));
    }

    #[test]
    fn run_select_input_with_arrow_escapes_stripped() {
        let opts = SelectPromptOptions {
            message: "Pick".into(),
            choices: vec![Choice::new("One", "1"), Choice::new("Two", "2")],
            initial: None,
            hint: None,
        };
        let mut stdin = Cursor::new(b"2\x1b[B\x1b[A\n");
        let mut stdout = Vec::new();
        let r = run_select(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "2");
    }

    #[test]
    fn strip_arrow_escapes_removes_ansi_sequences() {
        assert_eq!(strip_arrow_escapes("5"), "5");
        assert_eq!(strip_arrow_escapes("5\x1b[B\x1b[A"), "5");
        assert_eq!(strip_arrow_escapes("1\x1b[C\x1b[D"), "1");
    }

    #[test]
    fn strip_arrow_escapes_keeps_incomplete_escape() {
        let s = "a\x1b";
        assert_eq!(strip_arrow_escapes(s), "a\x1b");
    }

    #[test]
    fn parse_selection_by_number() {
        let opts = SelectPromptOptions {
            message: "".into(),
            choices: vec![Choice::new("A", "a"), Choice::new("B", "b")],
            initial: None,
            hint: None,
        };
        assert_eq!(parse_selection(&opts, "1").unwrap(), 0);
        assert_eq!(parse_selection(&opts, "2").unwrap(), 1);
    }

    #[test]
    fn parse_selection_by_title_case_insensitive() {
        let opts = SelectPromptOptions {
            message: "".into(),
            choices: vec![Choice::new("Apple", "a"), Choice::new("Banana", "b")],
            initial: None,
            hint: None,
        };
        assert_eq!(parse_selection(&opts, "BANANA").unwrap(), 1);
    }

    #[test]
    fn parse_selection_by_value() {
        let opts = SelectPromptOptions {
            message: "".into(),
            choices: vec![Choice::new("One", "v1"), Choice::new("Two", "v2")],
            initial: None,
            hint: None,
        };
        assert_eq!(parse_selection(&opts, "v2").unwrap(), 1);
    }

    #[test]
    fn parse_selection_invalid_falls_back_to_initial() {
        let opts = SelectPromptOptions {
            message: "".into(),
            choices: vec![Choice::new("A", "a"), Choice::new("B", "b")],
            initial: Some(1),
            hint: None,
        };
        assert_eq!(parse_selection(&opts, "xyz").unwrap(), 1);
    }

    #[test]
    fn parse_selection_number_out_of_range_falls_back() {
        let opts = SelectPromptOptions {
            message: "".into(),
            choices: vec![Choice::new("A", "a")],
            initial: Some(0),
            hint: None,
        };
        assert_eq!(parse_selection(&opts, "99").unwrap(), 0);
    }

    #[test]
    fn next_enabled_skips_disabled() {
        let mut a = Choice::new("A", "a");
        let mut b = Choice::new("B", "b");
        b.disabled = true;
        let mut c = Choice::new("C", "c");
        let choices = vec![a, b, c];
        assert_eq!(next_enabled(&choices, 0), 2);
        assert_eq!(next_enabled(&choices, 2), 2);
    }

    #[test]
    fn prev_enabled_skips_disabled() {
        let mut a = Choice::new("A", "a");
        let mut b = Choice::new("B", "b");
        b.disabled = true;
        let mut c = Choice::new("C", "c");
        let choices = vec![a, b, c];
        assert_eq!(prev_enabled(&choices, 2), 0);
        assert_eq!(prev_enabled(&choices, 0), 0);
    }

    #[test]
    fn read_byte_returns_first_byte() {
        let mut r = Cursor::new(b"ab");
        assert_eq!(read_byte(&mut r).unwrap(), b'a');
        assert_eq!(read_byte(&mut r).unwrap(), b'b');
    }

    #[test]
    fn read_byte_eof_returns_err() {
        let mut r = Cursor::new(b"");
        assert!(read_byte(&mut r).is_err());
    }
}
