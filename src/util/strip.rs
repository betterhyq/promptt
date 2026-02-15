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
}
