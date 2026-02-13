//! Number prompt (mirrors prompts/lib/elements/number).

use crate::util::style;
use colour::write_bold;
use std::io::{self, BufRead, Write};

/// Options for a number prompt.
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

/// Run a number prompt. Returns the number (or initial if empty and initial is set).
pub fn run_number<R: BufRead, W: Write>(
    opts: &NumberPromptOptions,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<f64> {
    let mut buf = Vec::new();
    write_bold!(&mut buf, "{}", opts.message).ok();
    let msg = String::from_utf8_lossy(&buf).into_owned();
    let initial_str = opts.initial.map(|n| {
        if opts.float {
            format!("{:.prec$}", n, prec = opts.round as usize)
        } else {
            format!("{}", n as i64)
        }
    }).unwrap_or_default();
    let symbol = style::symbol(false, false, false);
    let delim = style::delimiter(false);
    write!(stdout, "{} {} {} {}", symbol, msg, delim, initial_str)?;
    stdout.flush()?;
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let raw = line.trim();
    let value = if raw.is_empty() {
        opts.initial.unwrap_or(0.0)
    } else {
        let v = if opts.float {
            raw.parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, opts.error_msg.as_deref().unwrap_or("invalid number")))?
        } else {
            raw.parse::<i64>().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, opts.error_msg.as_deref().unwrap_or("invalid number")))? as f64
        };
        let v = round_n(v, opts.round);
        let v = opts.min.map(|m| v.max(m)).unwrap_or(v);
        let v = opts.max.map(|m| v.min(m)).unwrap_or(v);
        v
    };
    let displayed = if opts.float {
        format!("{:.prec$}", value, prec = opts.round as usize)
    } else {
        format!("{}", value as i64)
    };
    let done_symbol = style::symbol(true, false, false);
    let done_delim = style::delimiter(true);
    writeln!(stdout, "\r{} {} {} {}", done_symbol, msg, done_delim, displayed)?;
    stdout.flush()?;
    Ok(value)
}
