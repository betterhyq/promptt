//! Prompt type registry and runners.

use crate::elements::*;
use crate::util::style::InputStyle;
use std::io::{self, BufRead, Write};

/// Result value of a single prompt (string, bool, float, or list).
#[derive(Debug, Clone)]
pub enum PromptValue {
    String(String),
    Bool(bool),
    Float(f64),
    List(Vec<String>),
}

/// Question configuration for the sequential prompt flow.
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

/// Runs a prompt by `type_name`. Returns `Some(value)` on success or `None` on cancel.
#[inline]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn question_default_values() {
        let q = Question::default();
        assert!(q.name.is_empty());
        assert!(q.type_name.is_empty());
        assert!(q.message.is_empty());
        assert!(q.initial_text.is_none());
        assert!(q.initial_number.is_none());
        assert!(q.initial_bool.is_none());
        assert!(q.choices.is_none());
        assert!(q.separator.is_none());
        assert!(!q.float);
        assert_eq!(q.round, 2);
        assert!(q.min.is_none());
        assert!(q.max.is_none());
        assert!(q.active.is_none());
        assert!(q.inactive.is_none());
        assert!(q.hint.is_none());
    }

    #[test]
    fn run_prompt_text() {
        let q = Question {
            name: "name".into(),
            type_name: "text".into(),
            message: "Your name?".into(),
            initial_text: Some("default".into()),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"Alice\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        let val = out.unwrap();
        assert!(matches!(val, Some(PromptValue::String(s)) if s == "Alice"));
    }

    #[test]
    fn run_prompt_text_empty_uses_initial() {
        let q = Question {
            name: "x".into(),
            type_name: "text".into(),
            message: "Msg".into(),
            initial_text: Some("init".into()),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        let val = out.unwrap();
        assert!(matches!(val, Some(PromptValue::String(s)) if s == "init"));
    }

    #[test]
    fn run_prompt_confirm_yes() {
        let q = Question {
            name: "ok".into(),
            type_name: "confirm".into(),
            message: "Continue?".into(),
            initial_bool: Some(false),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"y\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::Bool(true))));
    }

    #[test]
    fn run_prompt_confirm_no() {
        let q = Question {
            name: "ok".into(),
            type_name: "confirm".into(),
            message: "Continue?".into(),
            initial_bool: Some(true),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"n\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::Bool(false))));
    }

    #[test]
    fn run_prompt_number_integer() {
        let q = Question {
            name: "n".into(),
            type_name: "number".into(),
            message: "Count?".into(),
            float: false,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"42\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::Float(x)) if x == 42.0));
    }

    #[test]
    fn run_prompt_number_float() {
        let q = Question {
            name: "n".into(),
            type_name: "number".into(),
            message: "Value?".into(),
            float: true,
            round: 2,
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"3.14\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        let v = out.unwrap().unwrap();
        if let PromptValue::Float(x) = v {
            assert!((x - 3.14).abs() < 0.01);
        } else {
            panic!("expected Float");
        }
    }

    #[test]
    fn run_prompt_toggle_on() {
        let q = Question {
            name: "t".into(),
            type_name: "toggle".into(),
            message: "Enable?".into(),
            initial_bool: Some(false),
            active: Some("on".into()),
            inactive: Some("off".into()),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"y\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::Bool(true))));
    }

    #[test]
    fn run_prompt_select_by_number() {
        let q = Question {
            name: "choice".into(),
            type_name: "select".into(),
            message: "Pick one".into(),
            choices: Some(vec![
                Choice::new("A", "a"),
                Choice::new("B", "b"),
                Choice::new("C", "c"),
            ]),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"2\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::String(s)) if s == "b"));
    }

    #[test]
    fn run_prompt_select_by_title() {
        let q = Question {
            name: "choice".into(),
            type_name: "select".into(),
            message: "Pick".into(),
            choices: Some(vec![
                Choice::new("Apple", "apple"),
                Choice::new("Banana", "banana"),
            ]),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"Apple\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::String(s)) if s == "apple"));
    }

    #[test]
    fn run_prompt_list_split() {
        let q = Question {
            name: "items".into(),
            type_name: "list".into(),
            message: "Items?".into(),
            separator: Some(",".into()),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"a, b , c\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::List(l)) if l == ["a", "b", "c"]));
    }

    #[test]
    fn run_prompt_password_returns_string() {
        let q = Question {
            name: "pwd".into(),
            type_name: "password".into(),
            message: "Password?".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"secret\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::String(s)) if s == "secret"));
    }

    #[test]
    fn run_prompt_invisible_returns_string() {
        let q = Question {
            name: "inv".into(),
            type_name: "invisible".into(),
            message: "Hidden?".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"value\n");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_ok());
        assert!(matches!(out.unwrap(), Some(PromptValue::String(s)) if s == "value"));
    }

    #[test]
    fn run_prompt_unknown_type_err() {
        let q = Question {
            name: "x".into(),
            type_name: "unknown_type".into(),
            message: "Msg".into(),
            ..Default::default()
        };
        let mut stdin = Cursor::new(b"");
        let mut stdout = Vec::new();
        let out = run_prompt(&q, &mut stdin, &mut stdout);
        assert!(out.is_err());
    }
}
