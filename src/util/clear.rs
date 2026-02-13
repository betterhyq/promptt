//! Clear prompt lines (mirrors prompts/lib/util/clear).

use ansi_escapes::{CursorTo, EraseLine, EraseLines};
use crate::util::strip::strip_ansi;

fn width(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

/// Clear the given prompt text over per_line columns (terminal width).
/// Returns the escape sequence to clear and move cursor to start of line.
pub fn clear(prompt: &str, per_line: usize) -> String {
    if per_line == 0 {
        return format!("{}{}", EraseLine, CursorTo::AbsoluteX(0));
    }
    let mut rows = 0u16;
    for line in prompt.split("\n") {
        let w = width(line);
        rows += 1 + (w.saturating_sub(1) / per_line) as u16;
    }
    format!("{}", EraseLines(rows))
}
