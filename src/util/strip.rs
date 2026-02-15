//! Stripping of ANSI escape codes from strings.

use regex::Regex;
use std::sync::OnceLock;

static ANSI_RE: OnceLock<Regex> = OnceLock::new();

fn ansi_re() -> &'static Regex {
    ANSI_RE.get_or_init(|| {
        Regex::new(
            r"\x1B(?:\[[0-?]*[ -/]*[@-~]|\][0-9;]*\x07|\[[?0-9;]*[a-zA-Z])|\x9B[0-?]*[ -/]*[@-~]",
        )
        .unwrap()
    })
}

/// Removes ANSI escape sequences from the string.
pub fn strip_ansi(s: &str) -> String {
    ansi_re().replace_all(s, "").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_ansi_removes_red() {
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
    }

    #[test]
    fn strip_ansi_plain_unchanged() {
        assert_eq!(strip_ansi("hello world"), "hello world");
    }

    #[test]
    fn strip_ansi_empty() {
        assert_eq!(strip_ansi(""), "");
    }

    #[test]
    fn strip_ansi_multiple_sequences() {
        let s = "\x1b[1mbold\x1b[0m \x1b[32mgreen\x1b[0m";
        assert_eq!(strip_ansi(s), "bold green");
    }

    #[test]
    fn strip_ansi_csi_style_codes() {
        assert_eq!(strip_ansi("\x1b[0;33myellow\x1b[0m"), "yellow");
    }

    #[test]
    fn strip_ansi_escape_at_start_only() {
        assert_eq!(strip_ansi("\x1b[0mhello"), "hello");
    }

    #[test]
    fn strip_ansi_escape_at_end_only() {
        assert_eq!(strip_ansi("hello\x1b[0m"), "hello");
    }

    #[test]
    fn strip_ansi_only_escapes() {
        assert_eq!(strip_ansi("\x1b[31m\x1b[0m"), "");
    }

    #[test]
    fn strip_ansi_mixed_keeps_order() {
        let s = "\x1b[1ma\x1b[0m\x1b[32mb\x1b[0m";
        assert_eq!(strip_ansi(s), "ab");
    }

    #[test]
    fn strip_ansi_unicode_unchanged() {
        assert_eq!(strip_ansi("日本語"), "日本語");
    }
}
