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
