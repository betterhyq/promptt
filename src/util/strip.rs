//! Strip ANSI escape codes from strings.

use regex::Regex;
use std::sync::OnceLock;

static ANSI_RE: OnceLock<Regex> = OnceLock::new();

fn ansi_re() -> &'static Regex {
    ANSI_RE.get_or_init(|| {
        // Strip CSI (ESC [) and OSC (ESC ]) and other common ANSI sequences
        Regex::new(r"\x1B(?:\[[0-?]*[ -/]*[@-~]|\][0-9;]*\x07|\[[?0-9;]*[a-zA-Z])|\x9B[0-?]*[ -/]*[@-~]")
            .unwrap()
    })
}

/// Returns string with ANSI escape sequences removed.
pub fn strip_ansi(s: &str) -> String {
    ansi_re().replace_all(s, "").into_owned()
}
