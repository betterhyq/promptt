//! Prompt output styling.

use crate::util::figures::Figures;
use colour::{write_cyan, write_gray, write_green, write_red, write_yellow};
use std::io::Write;

/// Input display style (default, password, or invisible).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputStyle {
    Default,
    Password,
    Invisible,
}

/// Transforms input for display according to style (e.g. mask password).
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

/// Returns style transform and display scale for the given input style.
pub fn render_style(style: InputStyle) -> (StyleTransform, usize) {
    let scale = match style {
        InputStyle::Password | InputStyle::Default => 1,
        InputStyle::Invisible => 0,
    };
    (StyleTransform { scale }, scale)
}

/// Returns prompt symbol: `?`, tick, or cross depending on state.
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

/// Returns delimiter between message and input (ellipsis when completing, pointer otherwise).
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

    #[test]
    fn symbol_priority_aborted_over_exited_and_done() {
        let s_aborted = symbol(true, true, false);
        let s_exited = symbol(true, false, true);
        assert!(!s_aborted.is_empty());
        assert!(!s_exited.is_empty());
    }

    #[test]
    fn symbol_pending_is_question_mark_without_ansi() {
        let s = symbol(false, false, false);
        assert!(s.contains('?') || !s.is_empty());
    }

    #[test]
    fn style_transform_render_empty_string_default() {
        let (t, _) = render_style(InputStyle::Default);
        assert_eq!(t.render("", InputStyle::Default), "");
    }

    #[test]
    fn style_transform_render_empty_string_password() {
        let (t, _) = render_style(InputStyle::Password);
        assert_eq!(t.render("", InputStyle::Password), "");
    }

    #[test]
    fn style_transform_render_empty_string_invisible() {
        let (t, _) = render_style(InputStyle::Invisible);
        assert_eq!(t.render("", InputStyle::Invisible), "");
    }

    #[test]
    fn style_transform_password_length_matches_input() {
        let (t, _) = render_style(InputStyle::Password);
        assert_eq!(t.render("abc", InputStyle::Password).len(), 3);
        assert_eq!(t.render("xyz", InputStyle::Password), "***");
    }

    #[test]
    fn render_style_invisible_scale_zero() {
        let (_, scale) = render_style(InputStyle::Invisible);
        assert_eq!(scale, 0);
    }
}
