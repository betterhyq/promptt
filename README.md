# promptt

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/promptt)
![Crates.io Total Downloads](https://img.shields.io/crates/d/promptt)
![docs.rs](https://img.shields.io/docsrs/promptt)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/betterhyq/promptt)
![GitHub Repo stars](https://img.shields.io/github/stars/betterhyq/promptt)
<!-- /automdrs -->

Lightweight interactive CLI prompts for Rust: text, confirm, number, select, toggle, list, password, invisible.

**[Full documentation →](https://betterhyq.github.io/promptt/)**

## Install

<!-- automdrs:cargo-add -->

```sh
cargo add promptt
```

<!-- /automdrs -->

## Usage

Build a list of `Question`s and run `prompt()`. Answers are `HashMap<String, PromptValue>`.

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

    let mut stdin = io::stdin().lock();
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

## Prompt types

| Type        | Result              | Options / notes                    |
|------------|---------------------|------------------------------------|
| `text`     | `PromptValue::String` | `initial_text`                     |
| `password` | `PromptValue::String` | Masked input                       |
| `invisible`| `PromptValue::String` | No echo                            |
| `number`   | `PromptValue::Float`  | `min`/`max`, `float`, `round`      |
| `confirm`  | `PromptValue::Bool`   | `initial_bool`                     |
| `toggle`   | `PromptValue::Bool`   | `active`/`inactive` labels         |
| `select`   | `PromptValue::String` | `choices` (title/value)            |
| `list`     | `PromptValue::List`   | `separator` (default `,`)          |

More in the [documentation](https://betterhyq.github.io/promptt/) and [docs.rs](https://docs.rs/promptt).

## License

<!-- automdrs:contributors author="YONGQI" license="MIT" -->
Published under the [MIT](./LICENSE) license.
Made by [@YONGQI](https://github.com/betterhyq) 💛
<br><br>
<a href="https://github.com/betterhyq/promptt/graphs/contributors">
<img src="https://contrib.rocks/image?repo=betterhyq/promptt" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->