use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Mode::Normal => "NORMAL",
            Mode::Command => "COMMAND",
        };
        write!(f, "{v}")
    }
}

pub struct State {
    pub mode: Mode,
    pub command_buffer: String,
    pub error_buffer: Option<String>,
    pub exit_requested: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            mode: Mode::Normal,
            command_buffer: String::new(),
            error_buffer: None,
            exit_requested: false,
        }
    }
}
