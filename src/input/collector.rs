use std::time::Duration;

use ratatui::crossterm::{self, event};
use tokio::sync::mpsc;

use crate::{event::Event, state::Mode};

use super::mapper::{CommandMapper, EventMapper, NormalMapper};

pub struct CollectorHandle {
    request_sender: mpsc::Sender<Request>,
}

impl CollectorHandle {
    pub async fn use_mode(&self, mode: Mode) {
        let mapper: Box<dyn EventMapper> = match mode {
            Mode::Normal => Box::new(NormalMapper),
            Mode::Command => Box::new(CommandMapper),
        };

        self.request_sender
            .send(Request::UseMapper(mapper))
            .await
            .unwrap()
    }
}

impl Drop for CollectorHandle {
    fn drop(&mut self) {
        self.request_sender.try_send(Request::Stop).unwrap()
    }
}

pub enum Request {
    Stop,
    UseMapper(Box<dyn EventMapper>),
}

struct State {
    mapper: Box<dyn EventMapper>,
    stop_requested: bool,
}

pub fn start_collector(event_sender: mpsc::Sender<Event>) -> CollectorHandle {
    let (request_tx, mut request_rx) = mpsc::channel(256);

    let handle = CollectorHandle {
        request_sender: request_tx,
    };

    tokio::spawn(async move {
        let mut state = State {
            mapper: Box::new(NormalMapper),
            stop_requested: false,
        };

        while !state.stop_requested {
            tokio::select! {
                Some(request) = request_rx.recv() => { handle_request(request, &mut state) }
                _ = handle_input(state.mapper.as_ref(), event_sender.clone()) => {}
            }
        }
    });

    handle
}

fn handle_request(request: Request, state: &mut State) {
    match request {
        Request::Stop => state.stop_requested = true,
        Request::UseMapper(mapper) => state.mapper = mapper,
    }
}

async fn handle_input(mapper: &dyn EventMapper, event_sender: mpsc::Sender<Event>) {
    if crossterm::event::poll(Duration::from_millis(100)).unwrap() {
        if let crossterm::event::Event::Key(key) = event::read().unwrap() {
            for e in mapper.map_from_key(key) {
                event_sender.send(e).await.unwrap()
            }
        }
    }
}
