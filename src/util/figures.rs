//! Unicode symbols for prompt UI (mirrors prompts/lib/util/figures).

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
