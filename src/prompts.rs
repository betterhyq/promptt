//! Prompt type registry and runners (mirrors prompts/lib/prompts).

use crate::elements::*;
use crate::util::style::InputStyle;
use std::io::{self, BufRead, Write};

/// Value returned from a single prompt (mirrors JS prompt return types).
#[derive(Debug, Clone)]
pub enum PromptValue {
    String(String),
    Bool(bool),
    Float(f64),
    List(Vec<String>),
}

/// Question for the prompt flow (mirrors the JS question object).
pub struct Question {
    pub name: String,
    pub type_name: String,
    pub message: String,
    pub initial_text: Option<String>,
    pub initial_number: Option<f64>,
    pub initial_bool: Option<bool>,
    pub choices: Option<Vec<Choice>>,
    pub style: InputStyle,
    pub separator: Option<String>,
    pub float: bool,
    pub round: u32,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub active: Option<String>,
    pub inactive: Option<String>,
    pub hint: Option<String>,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            name: String::new(),
            type_name: String::new(),
            message: String::new(),
            initial_text: None,
            initial_number: None,
            initial_bool: None,
            choices: None,
            style: InputStyle::Default,
            separator: None,
            float: false,
            round: 2,
            min: None,
            max: None,
            active: None,
            inactive: None,
            hint: None,
        }
    }
}

/// Run one prompt by type. Returns the value or None on cancel.
pub fn run_prompt<R: BufRead, W: Write>(
    q: &Question,
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<Option<PromptValue>> {
    match q.type_name.as_str() {
        "text" => {
            let opts = TextPromptOptions {
                message: q.message.clone(),
                initial: q.initial_text.clone(),
                style: q.style,
                error_msg: None,
            };
            run_text(&opts, stdin, stdout).map(|s| Some(PromptValue::String(s)))
        }
        "password" => {
            let opts = TextPromptOptions {
                message: q.message.clone(),
                initial: q.initial_text.clone(),
                style: InputStyle::Password,
                error_msg: None,
            };
            run_text(&opts, stdin, stdout).map(|s| Some(PromptValue::String(s)))
        }
        "invisible" => {
            let opts = TextPromptOptions {
                message: q.message.clone(),
                initial: q.initial_text.clone(),
                style: InputStyle::Invisible,
                error_msg: None,
            };
            run_text(&opts, stdin, stdout).map(|s| Some(PromptValue::String(s)))
        }
        "number" => {
            let opts = NumberPromptOptions {
                message: q.message.clone(),
                initial: q.initial_number,
                min: q.min,
                max: q.max,
                float: q.float,
                round: q.round,
                error_msg: None,
            };
            run_number(&opts, stdin, stdout).map(|n| Some(PromptValue::Float(n)))
        }
        "confirm" => {
            let opts = ConfirmPromptOptions {
                message: q.message.clone(),
                initial: q.initial_bool.unwrap_or(false),
                ..Default::default()
            };
            run_confirm(&opts, stdin, stdout).map(|b| Some(PromptValue::Bool(b)))
        }
        "toggle" => {
            let opts = TogglePromptOptions {
                message: q.message.clone(),
                initial: q.initial_bool.unwrap_or(false),
                active: q.active.clone().unwrap_or_else(|| "on".into()),
                inactive: q.inactive.clone().unwrap_or_else(|| "off".into()),
            };
            run_toggle(&opts, stdin, stdout).map(|b| Some(PromptValue::Bool(b)))
        }
        "select" => {
            let choices = q.choices.clone().unwrap_or_default();
            let opts = SelectPromptOptions {
                message: q.message.clone(),
                choices,
                initial: None,
                hint: q.hint.clone(),
            };
            run_select(&opts, stdin, stdout).map(|s| Some(PromptValue::String(s)))
        }
        "list" => {
            let sep = q.separator.as_deref().unwrap_or(",");
            let opts = TextPromptOptions {
                message: q.message.clone(),
                initial: q.initial_text.clone(),
                style: InputStyle::Default,
                error_msg: None,
            };
            run_text(&opts, stdin, stdout).map(|s| {
                let list = s.split(sep).map(|x| x.trim().to_string()).collect();
                Some(PromptValue::List(list))
            })
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("prompt type '{}' is not defined", q.type_name),
        )),
    }
}
