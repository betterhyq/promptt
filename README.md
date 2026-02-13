# promptt

![Crates.io Version](https://img.shields.io/crates/v/promptt)
![Crates.io Total Downloads](https://img.shields.io/crates/d/promptt)
![Crates.io License](https://img.shields.io/crates/l/promptt)

`promptt` is a lightweight, interactive CLI prompts library for Rust. It supports text, confirm, number, select, toggle, list, password, and invisible inputs—ideal for building terminal wizards, config generators, or any step-by-step CLI flow.

## Installation

Add this crate with Cargo:

```bash
cargo add promptt
```

## Usage

Define a list of `Question`s (each with a `name`, `type_name`, and `message`), then run them with `prompt()`. Answers are returned as a `HashMap<String, PromptValue>`.

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
            name: "confirm".into(),
            type_name: "confirm".into(),
            message: "Continue?".into(),
            initial_bool: Some(true),
            ..Default::default()
        },
    ];

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();
    let answers = prompt(&questions, &mut stdin, &mut stdout)?;

    if let Some(PromptValue::String(name)) = answers.get("username") {
        println!("Hello, {}!", name);
    }
    if let Some(PromptValue::Bool(yes)) = answers.get("confirm") {
        println!("You chose: {}", if *yes { "yes" } else { "no" });
    }

    Ok(())
}
```

### Supported prompt types

| Type        | Result           | Notes                                      |
|------------|------------------|--------------------------------------------|
| `text`     | `PromptValue::String` | Single-line input                      |
| `password` | `PromptValue::String` | Input hidden (masked)                   |
| `invisible`| `PromptValue::String` | Input hidden (no echo)                 |
| `number`   | `PromptValue::Float`  | Use `min`/`max`, `float`, `round` on `Question` |
| `confirm`  | `PromptValue::Bool`   | Yes/No; set `initial_bool` for default  |
| `toggle`   | `PromptValue::Bool`   | Use `active`/`inactive` for labels       |
| `select`   | `PromptValue::String` | Single choice from `choices`            |
| `list`     | `PromptValue::List`   | Multiple choices; use `separator` on `Question` |

## Contribution

<details>
  <summary>Local development</summary>

- Clone this repository
- Install the latest version of [Rust](https://rust-lang.org/)
- Run tests using `cargo test` or `cargo run`

</details>

## Credits

`promptt` has been inspired by several outstanding projects in the community:

- [@prompts](https://github.com/terkelg/prompts) - Lightweight, beautiful and user-friendly interactive prompts

## License

Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/betterhyq) 💛
