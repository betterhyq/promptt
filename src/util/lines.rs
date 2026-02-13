//! Line count for styled text (mirrors prompts/lib/util/lines).

use crate::util::strip::strip_ansi;

/// Number of lines the message takes when wrapped to per_line width.
pub fn lines_count(msg: &str, per_line: usize) -> usize {
    let s = strip_ansi(msg);
    if s.is_empty() {
        return 0;
    }
    let line_count = s.split("\n").count();
    if per_line == 0 {
        return line_count;
    }
    s.split("\n")
        .map(|l| (l.chars().count() + per_line - 1) / per_line)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_count_empty() {
        assert_eq!(lines_count("", 80), 0);
    }

    #[test]
    fn lines_count_single_short() {
        assert_eq!(lines_count("hello", 80), 1);
    }

    #[test]
    fn lines_count_per_line_zero() {
        assert_eq!(lines_count("a\nb\nc", 0), 3);
    }

    #[test]
    fn lines_count_wrap() {
        assert_eq!(lines_count("abcdefghij", 4), 3);
    }

    #[test]
    fn lines_count_exact_fit() {
        assert_eq!(lines_count("abcd", 4), 1);
    }

    #[test]
    fn lines_count_multiline_then_wrap() {
        assert_eq!(lines_count("ab\ncd", 2), 2);
    }

    #[test]
    fn lines_count_strips_ansi() {
        assert_eq!(lines_count("\x1b[31mred\x1b[0m", 80), 1);
    }
}
