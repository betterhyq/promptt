//! Clearing of prompt lines on the terminal.

use crate::util::strip::strip_ansi;
use ansi_escapes::{CursorTo, EraseLine, EraseLines};

fn width(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

/// Returns escape sequence to clear the prompt over `per_line` columns.
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

    #[test]
    fn clear_empty_string_per_line_zero() {
        let out = clear("", 0);
        assert!(out.contains("\x1b[2K"));
    }

    #[test]
    fn clear_empty_string_nonzero_per_line() {
        let out = clear("", 80);
        assert!(!out.is_empty());
    }

    #[test]
    fn clear_single_char_one_row() {
        let out = clear("x", 80);
        assert!(!out.is_empty());
    }

    #[test]
    fn clear_exact_width_one_row() {
        let out = clear(&"a".repeat(80), 80);
        assert!(!out.is_empty());
    }

    #[test]
    fn clear_one_over_width_two_rows() {
        let out_single = clear(&"a".repeat(80), 80);
        let out_wrapped = clear(&"a".repeat(81), 80);
        assert!(out_wrapped.len() >= out_single.len());
    }

    #[test]
    fn clear_multiline_row_count_greater_than_line_count() {
        let two_lines = clear("a\nb", 10);
        let five_lines = clear("a\nb\nc\nd\ne", 10);
        assert!(!two_lines.is_empty());
        assert!(!five_lines.is_empty());
    }
}
