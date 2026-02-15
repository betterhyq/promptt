//! Key-to-action mapping for prompt input.

/// Key event for line-based input.
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

/// Action a prompt element can handle.
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

/// Maps key to action. Returns `None` for raw input passthrough (e.g. normal typing).
#[inline]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn key_char(c: char) -> Key {
        Key {
            name: KeyName::Char(c),
            ctrl: false,
            meta: false,
        }
    }

    fn key_ctrl(c: char) -> Key {
        Key {
            name: KeyName::Char(c),
            ctrl: true,
            meta: false,
        }
    }

    #[test]
    fn return_submit() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Return,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Submit)
        );
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Enter,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Submit)
        );
    }

    #[test]
    fn backspace_delete() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Backspace,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Delete)
        );
    }

    #[test]
    fn delete_forward() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Delete,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::DeleteForward)
        );
    }

    #[test]
    fn escape_exit() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Escape,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Exit)
        );
    }

    #[test]
    fn arrow_keys() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Up,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Up)
        );
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Down,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Down)
        );
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Left,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Left)
        );
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Right,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Right)
        );
    }

    #[test]
    fn home_end() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Home,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Home)
        );
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::End,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::End)
        );
    }

    #[test]
    fn ctrl_a_first() {
        assert_eq!(key_action(&key_ctrl('a'), false), Some(PromptAction::First));
    }

    #[test]
    fn ctrl_c_abort() {
        assert_eq!(key_action(&key_ctrl('c'), false), Some(PromptAction::Abort));
    }

    #[test]
    fn ctrl_d_abort() {
        assert_eq!(key_action(&key_ctrl('d'), false), Some(PromptAction::Abort));
    }

    #[test]
    fn ctrl_e_last() {
        assert_eq!(key_action(&key_ctrl('e'), false), Some(PromptAction::Last));
    }

    #[test]
    fn ctrl_g_reset() {
        assert_eq!(key_action(&key_ctrl('g'), false), Some(PromptAction::Reset));
    }

    #[test]
    fn select_j_k() {
        assert_eq!(key_action(&key_char('j'), true), Some(PromptAction::Down));
        assert_eq!(key_action(&key_char('k'), true), Some(PromptAction::Up));
    }

    #[test]
    fn non_select_j_k_passthrough() {
        assert_eq!(key_action(&key_char('j'), false), None);
        assert_eq!(key_action(&key_char('k'), false), None);
    }

    #[test]
    fn regular_char_none() {
        assert_eq!(key_action(&key_char('x'), false), None);
    }

    #[test]
    fn tab_next() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Tab,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::Next)
        );
    }

    #[test]
    fn page_up_down() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::PageUp,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::PrevPage)
        );
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::PageDown,
                    ctrl: false,
                    meta: false
                },
                false
            ),
            Some(PromptAction::NextPage)
        );
    }

    #[test]
    fn meta_key_passthrough() {
        assert_eq!(
            key_action(
                &Key {
                    name: KeyName::Char('a'),
                    ctrl: false,
                    meta: true
                },
                false
            ),
            None
        );
    }

    #[test]
    fn ctrl_other_char_none() {
        assert_eq!(key_action(&key_ctrl('z'), false), None);
    }
}
