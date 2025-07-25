use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{event::Event, state::Mode};

pub trait EventMapper: Send + Sync + 'static {
    fn map_from_key(&self, key: KeyEvent) -> Option<Event>;
}

pub struct NormalMapper;

impl EventMapper for NormalMapper {
    fn map_from_key(&self, key: KeyEvent) -> Option<Event> {
        match key.code {
            KeyCode::Char(':') => Some(Event::UseMode(Mode::Command)),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Event::Exit)
            }
            _ => None,
        }
    }
}

pub struct CommandMapper;

impl EventMapper for CommandMapper {
    fn map_from_key(&self, key: KeyEvent) -> Option<Event> {
        match key.code {
            KeyCode::Esc => Some(Event::UseMode(Mode::Normal)),
            KeyCode::Enter => Some(Event::SubmitCommand),
            KeyCode::Char(ch) => Some(Event::AppendCommandBuffer(ch)),
            KeyCode::Backspace => Some(Event::PopCommandBuffer),
            _ => None,
        }
    }
}
