# Prompt Types

All prompts are configured via `Question`. Set `type_name` to one of the values below and fill the relevant fields.

## Overview

| type_name  | Result              | Main options |
|------------|---------------------|--------------|
| `text`     | `PromptValue::String` | `initial_text` |
| `password` | `PromptValue::String` | (masked) |
| `invisible`| `PromptValue::String` | (no echo) |
| `number`   | `PromptValue::Float`  | `min`, `max`, `float`, `round`, `initial_number` |
| `confirm`  | `PromptValue::Bool`   | `initial_bool` |
| `toggle`   | `PromptValue::Bool`   | `initial_bool`, `active`, `inactive` |
| `select`   | `PromptValue::String` | `choices`, `hint` |
| `list`     | `PromptValue::List`   | `separator` (default `,`) |

---

## text

Single-line input. Result: `PromptValue::String`.

```rust
Question {
    name: "username".into(),
    type_name: "text".into(),
    message: "What is your name?".into(),
    initial_text: Some("anonymous".into()),
    ..Default::default()
}
```

## password

Same as text but input is masked. Result: `PromptValue::String`.

## invisible

Same as text but input is not echoed. Result: `PromptValue::String`.

---

## number

Numeric input. Result: `PromptValue::Float`. Use `float: true` for decimals; `round` sets decimal places. Optional `min`/`max` clamp the value.

```rust
Question {
    name: "count".into(),
    type_name: "number".into(),
    message: "Enter a number (1–10)".into(),
    initial_number: Some(5.0),
    min: Some(1.0),
    max: Some(10.0),
    float: false,
    ..Default::default()
}
```

## confirm

Yes/No. Result: `PromptValue::Bool`. `initial_bool` sets the default.

```rust
Question {
    name: "proceed".into(),
    type_name: "confirm".into(),
    message: "Do you want to continue?".into(),
    initial_bool: Some(true),
    ..Default::default()
}
```

## toggle

On/off with customizable labels. Result: `PromptValue::Bool`. Use `active` and `inactive` for the two options (e.g. `"Yes"` / `"No"`).

```rust
Question {
    name: "notifications".into(),
    type_name: "toggle".into(),
    message: "Enable notifications?".into(),
    initial_bool: Some(false),
    active: Some("Yes".into()),
    inactive: Some("No".into()),
    ..Default::default()
}
```

## select

Single choice from a list. Result: `PromptValue::String` — the **value** of the chosen `Choice`. Use `Choice::new(title, value)` for each option. Optional `hint` for usage tip.

```rust
use promptt::Choice;

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
}
```

## list

User enters a line that is split by `separator` (default `,`). Result: `PromptValue::List` (trimmed strings).

```rust
Question {
    name: "tags".into(),
    type_name: "list".into(),
    message: "Enter tags (comma-separated)".into(),
    separator: Some(",".into()),
    ..Default::default()
}
```

Input like `a, b , c` yields `vec!["a", "b", "c"]`.
