//! Interactive CLI prompts: text, confirm, number, select, toggle, list, password, invisible.

mod elements;
mod prompts;
mod util;

pub use elements::{
    Choice, ConfirmPromptOptions, NumberPromptOptions, Prompt, SelectPromptOptions,
    TextPromptOptions, TogglePromptOptions,
};
pub use prompts::{PromptValue, Question, run_prompt};
pub use util::{Figures, InputStyle, clear, key_action, lines_count, render_style, strip_ansi};

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// Runs questions in sequence. Returns a name-to-value map. I/O via stdin/stdout.
pub fn prompt<R: BufRead, W: Write>(
    questions: &[Question],
    stdin: &mut R,
    stdout: &mut W,
) -> io::Result<HashMap<String, PromptValue>> {
    let mut answers = HashMap::with_capacity(questions.len());
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
        let questions = vec![Question {
            name: "skip".into(),
            type_name: String::new(),
            message: "Skipped?".into(),
            ..Default::default()
        }];
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

    #[test]
    fn prompt_multiple_empty_type_names_skip_all() {
        let questions = vec![
            Question {
                name: "a".into(),
                type_name: String::new(),
                message: "A?".into(),
                ..Default::default()
            },
            Question {
                name: "b".into(),
                type_name: String::new(),
                message: "B?".into(),
                ..Default::default()
            },
        ];
        let mut stdin = Cursor::new(b"");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        assert!(r.unwrap().is_empty());
    }

    #[test]
    fn prompt_answer_keys_match_question_names() {
        let questions = vec![
            Question {
                name: "first".into(),
                type_name: "text".into(),
                message: "First?".into(),
                ..Default::default()
            },
            Question {
                name: "second".into(),
                type_name: "text".into(),
                message: "Second?".into(),
                ..Default::default()
            },
        ];
        let mut stdin = Cursor::new(b"one\ntwo\n");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_ok());
        let answers = r.unwrap();
        assert_eq!(answers.get("first"), Some(&PromptValue::String("one".into())));
        assert_eq!(answers.get("second"), Some(&PromptValue::String("two".into())));
    }

    #[test]
    fn prompt_first_question_invalid_type_returns_err_immediately() {
        let questions = vec![
            Question {
                name: "x".into(),
                type_name: "bad".into(),
                message: "X?".into(),
                ..Default::default()
            },
            Question {
                name: "y".into(),
                type_name: "text".into(),
                message: "Y?".into(),
                ..Default::default()
            },
        ];
        let mut stdin = Cursor::new(b"ignored\n");
        let mut stdout = Vec::new();
        let r = prompt(&questions, &mut stdin, &mut stdout);
        assert!(r.is_err());
    }
}
