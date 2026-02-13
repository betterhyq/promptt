//! Prompt rendering utilities.

pub mod action;
pub mod clear;
pub mod figures;
pub mod lines;
pub mod style;
pub mod strip;

pub use action::key_action;
pub use clear::clear;
pub use figures::Figures;
pub use lines::lines_count;
pub use style::{render_style, InputStyle};
pub use strip::strip_ansi;
