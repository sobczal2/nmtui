use super::mode::Mode;

#[derive(Debug)]
pub enum CommandEvent {
    Put(char),
    Pop,
    Submit,
    EnterMode(Mode),
}
