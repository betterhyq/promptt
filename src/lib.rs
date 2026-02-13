//! Rust implementation of prompts/lib: interactive CLI prompts.
//!
//! Uses the same concepts as the JS version: multiple question types (text, confirm, number,
//! select, toggle, list, password, invisible) and a single `prompt()` flow that runs
//! a series of questions and returns a map of answers.

mod elements;
mod prompts;
mod util;

pub use elements::{
    Choice, ConfirmPromptOptions, NumberPromptOptions, Prompt, SelectPromptOptions,
    TextPromptOptions, TogglePromptOptions,
};
pub use prompts::{run_prompt, PromptValue, Question};
pub use util::{clear, key_action, lines_count, render_style, strip_ansi, Figures, InputStyle};

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// Run a series of questions and return a map of name -> PromptValue.
/// Uses stdin and stdout for I/O.
///
/// # Example
///
/// ```ignore
/// let questions = vec![
///     Question {
///         name: "name".into(),
///         type_name: "text".into(),
///         message: "Your name?".into(),
///         ..Default::default()
///     },
///     Question {
///         name: "ok".into(),
///         type_name: "confirm".into(),
///         message: "Continue?".into(),
///         initial_bool: Some(true),
///         ..Default::default()
///     },
/// ];
/// let answers = promptt::prompt(&questions, &mut stdin.lock(), &mut stdout)?;
/// ```
pub fn prompt<R: BufRead, W: Write>(
    questions: &[Question],
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<HashMap<String, PromptValue>> {
    let mut answers = HashMap::new();
    for q in questions {
        if q.type_name.is_empty() {
            continue;
        }
        if q.message.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "prompt message is required",
            ));
        }
        match run_prompt(q, stdin, stdout) {
            Ok(Some(value)) => {
                answers.insert(q.name.clone(), value);
            }
            Ok(None) => {}
            Err(e) => return Err(e),
        }
    }
    Ok(answers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_strip_ansi() {
        let s = "\x1b[31mred\x1b[0m";
        assert_eq!(strip_ansi(s), "red");
    }

    #[test]
    fn test_figures() {
        let f = Figures::new();
        assert!(!f.tick.is_empty());
        assert!(!f.cross.is_empty());
    }

    #[test]
    fn test_question_default() {
        let q = Question::default();
        assert!(q.type_name.is_empty());
        assert!(q.message.is_empty());
    }

    #[test]
    fn prompt_skips_empty_type_name() {
        let questions = vec![
            Question {
                name: "skip".into(),
                type_name: String::new(),
                message: "Skipped?".into(),
                ..Default::default()
            },
        ];
        let mut stdin = Cursor::new(b"");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        let answers = r.unwrap();
        assert!(answers.is_empty());
    }

    #[test]
    fn prompt_requires_message() {
        let questions = vec![Question {
            name: "x".into(),
            type_name: "text".into(),
            message: String::new(),
            ..Default::default()
        }];
        let mut stdin = Cursor::new(b"");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_err());
    }

    #[test]
    fn prompt_collects_answers() {
        let questions = vec![
            Question {
                name: "name".into(),
                type_name: "text".into(),
                message: "Name?".into(),
                ..Default::default()
            },
            Question {
                name: "ok".into(),
                type_name: "confirm".into(),
                message: "Ok?".into(),
                initial_bool: Some(false),
                ..Default::default()
            },
        ];
        let mut stdin = Cursor::new(b"Alice\ny\n");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        let answers = r.unwrap();
        assert_eq!(answers.len(), 2);
        match answers.get("name") {
            Some(PromptValue::String(s)) => assert_eq!(s, "Alice"),
            _ => panic!("expected name to be String(\"Alice\")"),
        }
        match answers.get("ok") {
            Some(PromptValue::Bool(b)) => assert!(*b),
            _ => panic!("expected ok to be Bool(true)"),
        }
    }

    #[test]
    fn prompt_returns_err_on_invalid_question_type() {
        let questions = vec![Question {
            name: "x".into(),
            type_name: "invalid_type".into(),
            message: "Msg".into(),
            ..Default::default()
        }];
        let mut stdin = Cursor::new(b"");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_err());
    }
}
