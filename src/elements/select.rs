//! Select prompt.

use crate::util::figures::Figures;
use crate::util::style;
use ansi_escapes::{CursorUp, EraseLines};
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

/// Read a single byte from a BufRead. Blocks until one is available.
fn read_byte<R: BufRead>(r: &mut R) -> io::Result<u8> {
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
fn next_enabled(choices: &[Choice], current: usize) -> usize {
    for i in (current + 1)..choices.len() {
        if !choices[i].disabled {
            return i;
        }
    }
    current
}

/// Returns previous enabled index when moving up, or same if none.
fn prev_enabled(choices: &[Choice], current: usize) -> usize {
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
fn strip_arrow_escapes(s: &str) -> String {
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
fn parse_selection(opts: &SelectPromptOptions, raw: &str) -> io::Result<usize> {
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
    fn write_choices(
        opts: &SelectPromptOptions,
        fig: &Figures,
        selected: usize,
        stdout: &mut dyn Write,
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
            writeln!(stdout, "  {} {} {}", num, prefix, c.title)?;
        }
        Ok(())
    }

    writeln!(stdout, "{} {} {}", symbol, msg, delim)?;
    write_choices(opts, fig, *selected, stdout)?;
    writeln!(stdout, "  {}", hint_styled)?;
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
            let up = n_lines as u16;
            write!(stdout, "{}", CursorUp(up))?;
            write!(stdout, "{}", EraseLines(up))?;
            writeln!(stdout, "{} {} {}", symbol, msg, delim)?;
            write_choices(opts, fig, *selected, stdout)?;
            writeln!(stdout, "  {}", hint_styled)?;
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
}
