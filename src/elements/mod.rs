//! Prompt elements (mirrors prompts/lib/elements).

mod confirm;
mod number;
mod prompt;
mod select;
mod text;
mod toggle;

pub use confirm::{run_confirm, ConfirmPromptOptions};
pub use number::{run_number, NumberPromptOptions};
pub use prompt::Prompt;
pub use select::{run_select, Choice, SelectPromptOptions};
pub use text::{run_text, TextPromptOptions};
pub use toggle::{run_toggle, TogglePromptOptions};
