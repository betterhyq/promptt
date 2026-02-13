//! Prompt output styling.

use crate::util::figures::Figures;
use colour::{write_cyan, write_gray, write_green, write_red, write_yellow};
use std::io::Write;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

/// Prompt symbol: ?, ✔, or ✖.
pub fn symbol(done: bool, aborted: bool, exited: bool) -> String {
    let fig = Figures::default();
    let mut buf = Vec::with_capacity(16);
    if aborted {
        write_red!(&mut buf, "{}", fig.cross).ok();
    } else if exited {
        write_yellow!(&mut buf, "{}", fig.cross).ok();
    } else if done {
        write_green!(&mut buf, "{}", fig.tick).ok();
    } else {
        write_cyan!(&mut buf, "?").ok();
    }
    String::from_utf8_lossy(&buf).into_owned()
}

/// Delimiter between message and input.
pub fn delimiter(completing: bool) -> String {
    let fig = Figures::default();
    let d = if completing {
        fig.ellipsis
    } else {
        fig.pointer_small
    };
    let mut buf = Vec::with_capacity(8);
    write_gray!(&mut buf, "{}", d).ok();
    String::from_utf8_lossy(&buf).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_style_default_scale_one() {
        let (transform, scale) = render_style(InputStyle::Default);
        assert_eq!(scale, 1);
        assert_eq!(transform.render("hello", InputStyle::Default), "hello");
    }

    #[test]
    fn render_style_password_masks() {
        let (transform, scale) = render_style(InputStyle::Password);
        assert_eq!(scale, 1);
        assert_eq!(transform.render("secret", InputStyle::Password), "******");
    }

    #[test]
    fn render_style_invisible_empty() {
        let (transform, scale) = render_style(InputStyle::Invisible);
        assert_eq!(scale, 0);
        assert_eq!(transform.render("hidden", InputStyle::Invisible), "");
    }

    #[test]
    fn symbol_done_contains_tick() {
        let s = symbol(true, false, false);
        assert!(!s.is_empty());
    }

    #[test]
    fn symbol_aborted_contains_cross() {
        let s = symbol(false, true, false);
        assert!(!s.is_empty());
    }

    #[test]
    fn symbol_exited_contains_cross() {
        let s = symbol(false, false, true);
        assert!(!s.is_empty());
    }

    #[test]
    fn symbol_pending_contains_question() {
        let s = symbol(false, false, false);
        assert!(!s.is_empty());
    }

    #[test]
    fn delimiter_completing_and_not_differ() {
        let d_false = delimiter(false);
        let d_true = delimiter(true);
        assert!(!d_false.is_empty());
        assert!(!d_true.is_empty());
    }
}
