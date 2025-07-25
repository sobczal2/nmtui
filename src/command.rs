use std::str::FromStr;

use anyhow::bail;

pub enum Command {
    Exit,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "q" | "wq" | "q!" | "wq!" => Ok(Self::Exit),
            _ => bail!("invalid command"),
        }
    }
}
