//! Unicode symbols for prompt UI.

#[derive(Clone)]
pub struct Figures {
    pub arrow_up: &'static str,
    pub arrow_down: &'static str,
    pub arrow_left: &'static str,
    pub arrow_right: &'static str,
    pub radio_on: &'static str,
    pub radio_off: &'static str,
    pub tick: &'static str,
    pub cross: &'static str,
    pub ellipsis: &'static str,
    pub pointer_small: &'static str,
    pub line: &'static str,
    pub pointer: &'static str,
}

impl Default for Figures {
    fn default() -> Self {
        let is_windows = cfg!(target_os = "windows");
        if is_windows {
            Figures {
                arrow_up: "↑",
                arrow_down: "↓",
                arrow_left: "←",
                arrow_right: "→",
                radio_on: "(*)",
                radio_off: "( )",
                tick: "√",
                cross: "×",
                ellipsis: "...",
                pointer_small: "»",
                line: "─",
                pointer: ">",
            }
        } else {
            Figures {
                arrow_up: "↑",
                arrow_down: "↓",
                arrow_left: "←",
                arrow_right: "→",
                radio_on: "◉",
                radio_off: "◯",
                tick: "✔",
                cross: "✖",
                ellipsis: "…",
                pointer_small: "›",
                line: "─",
                pointer: "❯",
            }
        }
    }
}

impl Figures {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn figures_all_non_empty() {
        let f = Figures::new();
        assert!(!f.arrow_up.is_empty());
        assert!(!f.arrow_down.is_empty());
        assert!(!f.arrow_left.is_empty());
        assert!(!f.arrow_right.is_empty());
        assert!(!f.radio_on.is_empty());
        assert!(!f.radio_off.is_empty());
        assert!(!f.tick.is_empty());
        assert!(!f.cross.is_empty());
        assert!(!f.ellipsis.is_empty());
        assert!(!f.pointer_small.is_empty());
        assert!(!f.line.is_empty());
        assert!(!f.pointer.is_empty());
    }

    #[test]
    fn figures_default_equals_new() {
        let a = Figures::default();
        let b = Figures::new();
        assert_eq!(a.tick, b.tick);
        assert_eq!(a.cross, b.cross);
    }
}
