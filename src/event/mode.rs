use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Command,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Mode::Normal => "NORMAL",
            Mode::Command => "COMMAND",
        };

        write!(f, "{content}")
    }
}
