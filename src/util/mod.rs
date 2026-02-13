//! Prompt rendering utilities.

pub mod action;
pub mod clear;
pub mod figures;
pub mod lines;
pub mod strip;
pub mod style;

pub use action::key_action;
pub use clear::clear;
pub use figures::Figures;
pub use lines::lines_count;
pub use strip::strip_ansi;
pub use style::{InputStyle, render_style};
