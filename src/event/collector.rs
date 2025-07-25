use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode};
use tokio::sync::{mpsc, oneshot};

use crate::event::mode::Mode;

use super::{command::CommandEvent, normal::NormalEvent};

pub struct CollectorHandle {
    pub normal_events: mpsc::Receiver<NormalEvent>,
    pub command_events: mpsc::Receiver<CommandEvent>,
    pub stop: oneshot::Sender<()>,
}
impl CollectorHandle {
    pub async fn clear(&mut self) {
        while !self.normal_events.is_empty() {
            self.normal_events.recv().await.unwrap();
        }

        while !self.command_events.is_empty() {
            self.command_events.recv().await.unwrap();
        }
    }
}

pub fn run_event_collector(poll_rate: Duration) -> CollectorHandle {
    let (normal_tx, normal_rx) = mpsc::channel(256);
    let (command_tx, command_rx) = mpsc::channel(256);
    let (stop_tx, stop_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        let mut stop_rx = stop_rx;

        loop {
            tokio::select! {
                _ = &mut stop_rx => {
                    break;
                }
                Ok(is_event_ready) = tokio::task::spawn_blocking(move || event::poll(poll_rate)) => {
                    if is_event_ready.unwrap_or(false) {
                        if let Ok(Event::Key(key)) = event::read() {
                            let command_event = match key.code {
                                event::KeyCode::Char(ch) => Some(CommandEvent::Put(ch)),
                                event::KeyCode::Esc => Some(CommandEvent::EnterMode(Mode::Normal)),
                                event::KeyCode::Enter => Some(CommandEvent::Submit),
                                event::KeyCode::Backspace => Some(CommandEvent::Pop),
                                _ => None,
                            };

                            if let Some(command_event) = command_event {
                                if command_tx.send(command_event).await.is_err() {
                                    break;
                                }
                            }

                            let normal_event = match key.code {
                                KeyCode::Char(':') => Some(NormalEvent::EnterMode(Mode::Command)),
                                _ => None,
                            };

                            if let Some(normal_event) = normal_event {
                                if normal_tx.send(normal_event).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    CollectorHandle {
        normal_events: normal_rx,
        command_events: command_rx,
        stop: stop_tx,
    }
}
