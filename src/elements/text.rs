//! Text prompt.

use crate::util::style::{self, InputStyle};
use colour::write_bold;
use std::io::{self, BufRead, Write};

/// Text prompt options.
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

/// Runs text prompt. Returns input or initial when empty.
pub fn run_text<R: BufRead, W: Write>(
    opts: &TextPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<String> {
    let (transform, _scale) = style::render_style(opts.style);
    let initial = opts.initial.as_deref().unwrap_or("");
    let mut output = Vec::with_capacity(opts.message.len() + 32);
    write_bold!(&mut output, "{}", opts.message).ok();
    let msg_styled = String::from_utf8_lossy(&output).into_owned();
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    // Do not pre-display initial value: it is not editable, so users could not change it.
    // Use initial only when the user submits with empty input.
    let prompt_line = format!("{} {} {} ", symbol, msg_styled, delim);
    write!(stdout, "{}", prompt_line)?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let value = line.trim().to_string();
    let value = if value.is_empty() {
        initial.to_string()
    } else {
        value
    };
    let rendered = transform.render(&value, opts.style);
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(
        stdout,
        "\r{} {} {} {}",
        done_symbol, msg_styled, done_delim, rendered
    )?;
    stdout.flush()?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::style::InputStyle;
    use std::io::Cursor;

    #[test]
    fn text_prompt_options_default() {
        let opts = TextPromptOptions::default();
        assert!(opts.message.is_empty());
        assert!(opts.initial.is_none());
        assert_eq!(opts.style, InputStyle::Default);
        assert!(opts.error_msg.is_some());
    }

    #[test]
    fn run_text_returns_entered_value() {
        let opts = TextPromptOptions {
            message: "Name?".into(),
            initial: None,
            style: InputStyle::Default,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"Bob\n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "Bob");
    }

    #[test]
    fn run_text_empty_uses_initial() {
        let opts = TextPromptOptions {
            message: "Name?".into(),
            initial: Some("default".into()),
            style: InputStyle::Default,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "default");
    }

    #[test]
    fn run_text_trims_input() {
        let opts = TextPromptOptions {
            message: "X?".into(),
            initial: None,
            style: InputStyle::Default,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"  spaced  \n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "spaced");
    }

    #[test]
    fn run_text_password_style_masks_output() {
        let opts = TextPromptOptions {
            message: "Secret?".into(),
            initial: None,
            style: InputStyle::Password,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"hello\n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "hello");
        let out = String::from_utf8(stdout).unwrap();
        assert!(out.contains("*****"));
    }

    #[test]
    fn run_text_empty_input_no_initial_returns_empty_string() {
        let opts = TextPromptOptions {
            message: "Name?".into(),
            initial: None,
            style: InputStyle::Default,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "");
    }

    #[test]
    fn run_text_invisible_style_returns_value_but_hides_in_output() {
        let opts = TextPromptOptions {
            message: "Hidden?".into(),
            initial: None,
            style: InputStyle::Invisible,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"secret\n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "secret");
        let out = String::from_utf8(stdout).unwrap();
        assert!(!out.contains("secret"));
    }

    #[test]
    fn run_text_whitespace_only_treated_as_empty_uses_initial() {
        let opts = TextPromptOptions {
            message: "X?".into(),
            initial: Some("default".into()),
            style: InputStyle::Default,
            error_msg: None,
        };
        let mut stdin = Cursor::new(b"   \n");
        let mut stdout = Vec::new();
        let r = run_text(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), "default");
    }
}
