//! Styling for prompt output (mirrors prompts/lib/util/style).

use crate::util::figures::Figures;
use colour::{write_cyan, write_gray, write_green, write_red, write_yellow};
use std::io::Write;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputStyle {
    Default,
    Password,
    Invisible,
}

pub struct StyleTransform {
    pub scale: usize,
}

impl StyleTransform {
    pub fn render(&self, input: &str, style: InputStyle) -> String {
        match style {
            InputStyle::Password => "*".repeat(input.len()),
            InputStyle::Invisible => String::new(),
            InputStyle::Default => input.to_string(),
        }
    }
}

pub fn render_style(style: InputStyle) -> (StyleTransform, usize) {
    let scale = match style {
        InputStyle::Password | InputStyle::Default => 1,
        InputStyle::Invisible => 0,
    };
    (StyleTransform { scale }, scale)
}

/// Symbol before the prompt (?, ✔, ✖).
pub fn symbol(done: bool, aborted: bool, exited: bool) -> String {
    let mut buf = Vec::new();
    if aborted {
        write_red!(&mut buf, "{}", Figures::new().cross).ok();
    } else if exited {
        write_yellow!(&mut buf, "{}", Figures::new().cross).ok();
    } else if done {
        write_green!(&mut buf, "{}", Figures::new().tick).ok();
    } else {
        write_cyan!(&mut buf, "?").ok();
    }
    String::from_utf8_lossy(&buf).into_owned()
}

/// Delimiter between message and input.
pub fn delimiter(completing: bool) -> String {
    let mut buf = Vec::new();
    let fig = Figures::new();
    let d = if completing { fig.ellipsis } else { fig.pointer_small };
    write_gray!(&mut buf, "{}", d).ok();
    String::from_utf8_lossy(&buf).into_owned()
}
