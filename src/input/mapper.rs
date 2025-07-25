use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{event::Event, state::Mode};

pub trait EventMapper: Send + Sync + 'static {
    fn map_from_key(&self, key: KeyEvent) -> Vec<Event>;
}

pub struct NormalMapper;

impl EventMapper for NormalMapper {
    fn map_from_key(&self, key: KeyEvent) -> Vec<Event> {
        match key.code {
            KeyCode::Char(':') => vec![
                Event::UseMode(Mode::Command),
                Event::AppendCommandBuffer(':'),
            ],
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                vec![Event::Exit]
            }
            _ => vec![],
        }
    }
}

pub struct CommandMapper;

impl EventMapper for CommandMapper {
    fn map_from_key(&self, key: KeyEvent) -> Vec<Event> {
        match key.code {
            KeyCode::Esc => vec![Event::UseMode(Mode::Normal)],
            KeyCode::Enter => vec![Event::SubmitCommand],
            KeyCode::Char(ch) => vec![Event::AppendCommandBuffer(ch)],
            KeyCode::Backspace => vec![Event::PopCommandBuffer],
            _ => vec![],
        }
    }
}
