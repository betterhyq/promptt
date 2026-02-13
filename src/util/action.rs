//! Map key events to prompt actions (mirrors prompts/lib/util/action).

/// Represents a key event (simplified for line-based input).
#[derive(Debug, Clone)]
pub struct Key {
    pub name: KeyName,
    pub ctrl: bool,
    pub meta: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyName {
    Char(char),
    Return,
    Enter,
    Backspace,
    Delete,
    Abort,
    Escape,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Unknown,
}

/// Action name that a prompt element can handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptAction {
    First,
    Last,
    Abort,
    Reset,
    Submit,
    Delete,
    DeleteForward,
    Exit,
    Next,
    NextPage,
    PrevPage,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
}

/// Map key to action. Returns None if the key should be passed as raw input.
pub fn key_action(key: &Key, is_select: bool) -> Option<PromptAction> {
    if key.meta && key.name != KeyName::Escape {
        return None;
    }
    if key.ctrl {
        return match key.name {
            KeyName::Char('a') => Some(PromptAction::First),
            KeyName::Char('c') => Some(PromptAction::Abort),
            KeyName::Char('d') => Some(PromptAction::Abort),
            KeyName::Char('e') => Some(PromptAction::Last),
            KeyName::Char('g') => Some(PromptAction::Reset),
            _ => None,
        };
    }
    if is_select {
        if let KeyName::Char('j') = key.name {
            return Some(PromptAction::Down);
        }
        if let KeyName::Char('k') = key.name {
            return Some(PromptAction::Up);
        }
    }
    match &key.name {
        KeyName::Return | KeyName::Enter => Some(PromptAction::Submit),
        KeyName::Backspace => Some(PromptAction::Delete),
        KeyName::Delete => Some(PromptAction::DeleteForward),
        KeyName::Abort => Some(PromptAction::Abort),
        KeyName::Escape => Some(PromptAction::Exit),
        KeyName::Tab => Some(PromptAction::Next),
        KeyName::PageDown => Some(PromptAction::NextPage),
        KeyName::PageUp => Some(PromptAction::PrevPage),
        KeyName::Home => Some(PromptAction::Home),
        KeyName::End => Some(PromptAction::End),
        KeyName::Up => Some(PromptAction::Up),
        KeyName::Down => Some(PromptAction::Down),
        KeyName::Left => Some(PromptAction::Left),
        KeyName::Right => Some(PromptAction::Right),
        _ => None,
    }
}
