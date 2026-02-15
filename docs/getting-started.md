# Getting Started

## Installation

Add `promptt` to your `Cargo.toml`:

```bash
cargo add promptt
```

Or add manually:

```toml
[dependencies]
promptt = "1"
```

## Minimal example

Each prompt is a `Question` with at least `name`, `type_name`, and `message`. Run them with `prompt()`; you get back a `HashMap<String, PromptValue>`.

```rust
use promptt::{prompt, Question, PromptValue};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let questions = vec![
        Question {
            name: "username".into(),
            type_name: "text".into(),
            message: "What is your name?".into(),
            ..Default::default()
        },
        Question {
            name: "ok".into(),
            type_name: "confirm".into(),
            message: "Continue?".into(),
            initial_bool: Some(true),
            ..Default::default()
        },
    ];

    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let answers = prompt(&questions, &mut stdin, &mut stdout)?;

    if let Some(PromptValue::String(name)) = answers.get("username") {
        println!("Hello, {}!", name);
    }
    if let Some(PromptValue::Bool(yes)) = answers.get("ok") {
        println!("You chose: {}", if *yes { "yes" } else { "no" });
    }

    Ok(())
}
```

## Result type: `PromptValue`

Answers are one of:

- `PromptValue::String(String)` — text, password, invisible, select
- `PromptValue::Bool(bool)` — confirm, toggle
- `PromptValue::Float(f64)` — number
- `PromptValue::List(Vec<String>)` — list (with optional `separator`)

See [Prompt Types](/prompt-types) for each question type and its options.
