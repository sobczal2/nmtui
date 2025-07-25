use crate::{
    event::Event,
    input::collector::{CollectorHandle, start_collector},
    state::{Mode, Page, State},
    ui::{
        command_input::render_command_input, header::render_header, layout::main_layout,
        main_block::render_main_block,
    },
};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Padding},
};
use tokio::sync::mpsc;
use tui_big_text::{BigText, PixelSize};

pub struct App<'a> {
    state: State<'a>,
    collector_handle: CollectorHandle,
    event_sender: mpsc::Sender<Event>,
    event_receiver: mpsc::Receiver<Event>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let (event_sender, event_receiver) = mpsc::channel(256);
        let collector_handle = start_collector(event_sender.clone());
        Self {
            state: Default::default(),
            collector_handle,
            event_sender,
            event_receiver,
        }
    }

    pub async fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) {
        while !self.state.exit_requested {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            self.clear_temp_state();
            self.handle_event().await;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = main_layout(frame.area());
        render_header(frame, layout[0]);
        render_command_input(self.state.command_buffer.clone(), frame, layout[1]);
        render_main_block(frame, self.state.mode, layout[2]);
    }

    async fn handle_event(&mut self) {
        match self.event_receiver.recv().await.expect("channel closed") {
            Event::Exit => self.state.exit_requested = true,
            Event::UseMode(mode) => self.switch_mode(mode).await,
            Event::SubmitCommand => self.handle_command().await,
            Event::AppendCommandBuffer(ch) => self.state.command_buffer.push(ch),
            Event::PopCommandBuffer => {
                self.state.command_buffer.pop();
                if self.state.command_buffer.is_empty() {
                    self.switch_mode(Mode::Normal).await
                }
            }
        }
    }

    async fn switch_mode(&mut self, mode: Mode) {
        self.state.mode = mode;
        self.collector_handle.use_mode(mode).await;
        self.clear_mode_state();
    }

    fn clear_mode_state(&mut self) {
        self.state.command_buffer.clear();
    }

    fn clear_temp_state(&mut self) {
        self.state.error_buffer = None;
    }

    async fn handle_command(&mut self) {
        let event = self.state.command_buffer.parse().unwrap();
        self.event_sender.send(event).await.unwrap();
    }

    async fn change_page(&mut self, page: Page) {
        match page {
            Page::Devices => todo!(),
        }
    }
}
