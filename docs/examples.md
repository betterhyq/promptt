# Examples

Full example using several prompt types. Run the [demo](https://github.com/betterhyq/promptt/tree/main/demo) with `cargo run -p demo` from the repo.

```rust
use promptt::{prompt, Choice, PromptValue, Question};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();

    let questions: Vec<Question> = vec![
        Question {
            name: "username".into(),
            type_name: "text".into(),
            message: "What is your name?".into(),
            initial_text: Some("anonymous".into()),
            ..Default::default()
        },
        Question {
            name: "proceed".into(),
            type_name: "confirm".into(),
            message: "Do you want to continue?".into(),
            initial_bool: Some(true),
            ..Default::default()
        },
        Question {
            name: "count".into(),
            type_name: "number".into(),
            message: "Enter a number (1–10)".into(),
            initial_number: Some(5.0),
            min: Some(1.0),
            max: Some(10.0),
            float: false,
            ..Default::default()
        },
        Question {
            name: "fruit".into(),
            type_name: "select".into(),
            message: "Pick a fruit".into(),
            choices: Some(vec![
                Choice::new("Apple", "apple"),
                Choice::new("Banana", "banana"),
                Choice::new("Cherry", "cherry"),
            ]),
            hint: Some("Use arrow keys or type to search".into()),
            ..Default::default()
        },
        Question {
            name: "notifications".into(),
            type_name: "toggle".into(),
            message: "Enable notifications?".into(),
            initial_bool: Some(false),
            active: Some("Yes".into()),
            inactive: Some("No".into()),
            ..Default::default()
        },
        Question {
            name: "tags".into(),
            type_name: "list".into(),
            message: "Enter tags (comma-separated)".into(),
            separator: Some(",".into()),
            ..Default::default()
        },
    ];

    writeln!(stdout, "--- promptt demo ---\n")?;
    stdout.flush()?;

    let answers = prompt(&questions, &mut stdin, &mut stdout)?;

    writeln!(stdout, "\n--- your answers ---")?;
    for (name, value) in &answers {
        let s = match value {
            PromptValue::String(v) => v.clone(),
            PromptValue::Bool(v) => v.to_string(),
            PromptValue::Float(v) => v.to_string(),
            PromptValue::List(v) => v.join(", "),
        };
        writeln!(stdout, "  {}: {}", name, s)?;
    }
    stdout.flush()?;

    Ok(())
}
```

## Handling answers

```rust
// String (text, password, invisible, select)
if let Some(PromptValue::String(s)) = answers.get("username") {
    println!("Name: {}", s);
}

// Bool (confirm, toggle)
if let Some(PromptValue::Bool(b)) = answers.get("proceed") {
    if *b { /* continue */ }
}

// Float (number)
if let Some(PromptValue::Float(n)) = answers.get("count") {
    println!("Count: {}", n);
}

// List (list)
if let Some(PromptValue::List(tags)) = answers.get("tags") {
    for tag in tags {
        println!("Tag: {}", tag);
    }
}
```
