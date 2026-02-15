//! Number prompt.

use crate::util::style;
use colour::write_bold;
use std::io::{self, BufRead, Write};

/// Number prompt options.
pub struct NumberPromptOptions {
    pub message: String,
    pub initial: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub float: bool,
    pub round: u32,
    pub error_msg: Option<String>,
}

impl Default for NumberPromptOptions {
    fn default() -> Self {
        Self {
            message: String::new(),
            initial: None,
            min: None,
            max: None,
            float: false,
            round: 2,
            error_msg: Some("Please Enter A Valid Value".into()),
        }
    }
}

fn round_n(x: f64, n: u32) -> f64 {
    let factor = 10_f64.powi(n as i32);
    (x * factor).round() / factor
}

/// Runs number prompt. Returns value or initial/0 when empty.
pub fn run_number<R: BufRead, W: Write>(
    opts: &NumberPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<f64> {
    let mut buf = Vec::with_capacity(opts.message.len() + 32);
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    // Do not pre-display initial value: it is not editable, so users could not change it.
    // Use initial only when the user submits with empty input.
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    write!(stdout, "{} {} {} ", symbol, msg, delim)?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let raw = line.trim();
    let value = if raw.is_empty() {
        opts.initial.unwrap_or(0.0)
    } else {
        let v = if opts.float {
            raw.parse::<f64>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    opts.error_msg.as_deref().unwrap_or("invalid number"),
                )
            })?
        } else {
            raw.parse::<i64>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    opts.error_msg.as_deref().unwrap_or("invalid number"),
                )
            })? as f64
        };
        let v = round_n(v, opts.round);
        let v = opts.min.map_or(v, |m| v.max(m));
        opts.max.map_or(v, |m| v.min(m))
    };
    let displayed = if opts.float {
        format!("{:.prec$}", value, prec = opts.round as usize)
    } else {
        format!("{}", value as i64)
    };
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(
        stdout,
        "\r{} {} {} {}",
        done_symbol, msg, done_delim, displayed
    )?;
    stdout.flush()?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn number_prompt_options_default() {
        let opts = NumberPromptOptions::default();
        assert!(opts.message.is_empty());
        assert!(opts.initial.is_none());
        assert!(opts.min.is_none());
        assert!(opts.max.is_none());
        assert!(!opts.float);
        assert_eq!(opts.round, 2);
        assert!(opts.error_msg.is_some());
    }

    #[test]
    fn run_number_integer() {
        let opts = NumberPromptOptions {
            message: "Count?".into(),
            float: false,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"100\n");
        let mut stdout = Vec::new();
        let r = run_number(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 100.0);
    }

    #[test]
    fn run_number_float() {
        let opts = NumberPromptOptions {
            message: "Value?".into(),
            float: true,
            round: 2,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"2.5\n");
        let mut stdout = Vec::new();
        let r = run_number(&opts, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert!((r.unwrap() - 2.5).abs() < 0.001);
    }

    #[test]
    fn run_number_empty_uses_initial() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            initial: Some(7.0),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        assert_eq!(run_number(&opts, &mut stdin, &mut stdout).unwrap(), 7.0);
    }

    #[test]
    fn run_number_empty_no_initial_defaults_zero() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        assert_eq!(run_number(&opts, &mut stdin, &mut stdout).unwrap(), 0.0);
    }

    #[test]
    fn run_number_min_clamp() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            min: Some(10.0),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"5\n");
        let mut stdout = Vec::new();
        assert_eq!(run_number(&opts, &mut stdin, &mut stdout).unwrap(), 10.0);
    }

    #[test]
    fn run_number_max_clamp() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            max: Some(10.0),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"99\n");
        let mut stdout = Vec::new();
        assert_eq!(run_number(&opts, &mut stdin, &mut stdout).unwrap(), 10.0);
    }

    #[test]
    fn run_number_invalid_returns_err() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"not a number\n");
        let mut stdout = Vec::new();
        assert!(run_number(&opts, &mut stdin, &mut stdout).is_err());
    }

    #[test]
    fn run_number_float_invalid_returns_err() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            float: true,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"abc\n");
        let mut stdout = Vec::new();
        assert!(run_number(&opts, &mut stdin, &mut stdout).is_err());
    }

    #[test]
    fn run_number_invalid_uses_error_msg_when_none() {
        let opts = NumberPromptOptions {
            message: "N?".into(),
            error_msg: None,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"x\n");
        let mut stdout = Vec::new();
        let r = run_number(&opts, &mut stdin, &mut stdout);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "invalid number");
    }
}
