use std::str::FromStr;

use anyhow::bail;

use crate::state::Mode;

pub enum Event {
    Exit,
    UseMode(Mode),
    SubmitCommand,
    AppendCommandBuffer(char),
    PopCommandBuffer,
}

impl FromStr for Event {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "q" | "wq" | "q!" | "wq!" => Ok(Self::Exit),
            _ => bail!("invalid event"),
        }
    }
}
