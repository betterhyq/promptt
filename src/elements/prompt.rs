//! Base prompt. Line-based I/O via stdout/stdin.

use ansi_escapes::Beep;
use std::io::{self, Write};

/// Base prompt behavior (output, beep).
pub struct Prompt;

impl Prompt {
    pub fn bell(out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{}", Beep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_bell_writes_to_output() {
        let mut buf = Vec::new();
        let r = Prompt::bell(&mut buf);
        assert!(r.is_ok());
        assert!(!buf.is_empty());
    }
}
