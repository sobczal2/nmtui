use crate::{
    event::Event,
    input::collector::{CollectorHandle, start_collector},
    state::{Mode, State},
};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Padding},
};
use tokio::sync::mpsc;
use tui_big_text::{BigText, PixelSize};

pub struct App {
    state: State,
    collector_handle: CollectorHandle,
    event_sender: mpsc::Sender<Event>,
    event_receiver: mpsc::Receiver<Event>,
}

impl App {
    pub fn new() -> App {
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
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(10),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let title_block = Block::new().padding(Padding::uniform(1));
        let title = BigText::builder()
            .pixel_size(PixelSize::Full)
            .lines(vec!["NMVI".fg(Color::Rgb(255, 204, 0)).into()])
            .alignment(Alignment::Center)
            .build();

        frame.render_widget(title, title_block.inner(layout[0]));

        let cmd_block = Block::bordered().border_style(Style::new().fg(Color::Rgb(235, 91, 0)));
        frame.render_widget(cmd_block.clone(), layout[1]);

        let cmd = Line::from(format!(
            "{}{}",
            if self.state.mode == Mode::Command {
                ":"
            } else {
                ""
            },
            self.state.command_buffer.clone()
        ));
        frame.render_widget(cmd, cmd_block.inner(layout[1]));

        let mode = Line::from(self.state.mode.to_string());
        let block = Block::bordered()
            .title_bottom(mode.left_aligned())
            .border_set(border::THICK)
            .fg(Color::White);
        frame.render_widget(block, layout[2]);
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
}
