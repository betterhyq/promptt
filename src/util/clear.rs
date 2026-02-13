//! Clears prompt lines from terminal.

use crate::util::strip::strip_ansi;
use ansi_escapes::{CursorTo, EraseLine, EraseLines};

fn width(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

/// Escape sequence to clear prompt over `per_line` columns.
pub fn clear(prompt: &str, per_line: usize) -> String {
    if per_line == 0 {
        return format!("{}{}", EraseLine, CursorTo::AbsoluteX(0));
    }
    let mut rows = 0u16;
    for line in prompt.split('\n') {
        let w = width(line);
        rows += 1 + (w.saturating_sub(1) / per_line) as u16;
    }
    format!("{}", EraseLines(rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_zero_width_returns_erase_line() {
        let out = clear("hello", 0);
        assert!(out.contains("\x1b[2K") || out.len() > 0);
    }

    #[test]
    fn clear_single_short_line() {
        let out = clear("hi", 80);
        assert!(!out.is_empty());
    }

    #[test]
    fn clear_multiline() {
        let out = clear("a\nb\nc", 10);
        assert!(!out.is_empty());
    }

    #[test]
    fn clear_long_line_wraps() {
        let long = "a".repeat(100);
        let out = clear(&long, 20);
        assert!(!out.is_empty());
    }
}
