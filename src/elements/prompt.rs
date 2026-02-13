//! Base prompt (mirrors prompts/lib/elements/prompt).
//! Line-based: no raw TTY; uses stdout for display and stdin read_line for input.

use ansi_escapes::Beep;
use std::io::{self, Write};

/// Base behavior for all prompts: write output, optionally beep.
pub struct Prompt;

impl Prompt {
    pub fn bell(out: &mut dyn Write) -> io::Result<()> {
        write!(out, "{}", Beep)
    }
}
