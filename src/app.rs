use std::{borrow::Cow, time::Duration};

use crate::{
    command::Command,
    event::{
        collector::{CollectorHandle, run_event_collector},
        command::CommandEvent,
        mode::Mode,
        normal::NormalEvent,
    },
};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{Block, Padding},
};
use tui_big_text::{BigText, PixelSize};

pub struct App {
    mode: Mode,
    command_buffer: String,
    error_text: Option<String>,
    collector_handle: CollectorHandle,
    should_exit: bool,
}

impl App {
    pub fn new() -> App {
        Self {
            mode: Mode::Normal,
            command_buffer: String::new(),
            error_text: None,
            collector_handle: run_event_collector(Duration::from_millis(250)),
            should_exit: false,
        }
    }

    pub async fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            self.clear_temp_state();
            self.handle_commands().await;
        }

        self.collector_handle.stop.send(()).unwrap();
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(10),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(if self.error_text.is_some() { 1 } else { 0 }),
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
            if self.mode == Mode::Command { ":" } else { "" },
            self.command_buffer.clone()
        ));
        frame.render_widget(cmd, cmd_block.inner(layout[1]));

        let mode = Line::from(self.mode.to_string());
        let block = Block::bordered()
            .title_bottom(mode.left_aligned())
            .border_set(border::THICK)
            .fg(Color::White);
        frame.render_widget(block, layout[2]);

        let error = Line::from(self.error_text.as_deref().unwrap_or("")).fg(Color::Red);
        frame.render_widget(error, layout[3]);
    }

    async fn handle_commands(&mut self) {
        match self.mode {
            Mode::Normal => {
                let event = self
                    .collector_handle
                    .normal_events
                    .recv()
                    .await
                    .expect("channel closed");
                self.handle_normal_event(event);
            }
            Mode::Command => {
                let event = self
                    .collector_handle
                    .command_events
                    .recv()
                    .await
                    .expect("channel closed");
                self.handle_command_event(event);
            }
        }

        self.collector_handle.clear().await
    }

    fn handle_command_event(&mut self, event: CommandEvent) {
        match event {
            CommandEvent::Put(ch) => self.command_buffer.push(ch),
            CommandEvent::Submit => {
                match self.command_buffer.parse() {
                    Ok(command) => self.execute_command(command),
                    Err(e) => self.emit_error(e),
                };
                self.command_buffer.clear();
                self.mode = Mode::Normal;
            }
            CommandEvent::EnterMode(mode) => {
                self.command_buffer.clear();
                self.mode = mode;
            }
            CommandEvent::Pop => {
                self.command_buffer.pop();
                if self.command_buffer.is_empty() {
                    self.mode = Mode::Normal;
                }
            }
        }
    }
    fn handle_normal_event(&mut self, event: NormalEvent) {
        match event {
            NormalEvent::EnterMode(mode) => self.mode = mode,
        }
    }
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Exit => self.should_exit = true,
        }
    }

    fn emit_error(&mut self, error: anyhow::Error) {
        self.error_text = Some(error.to_string());
    }

    fn clear_temp_state(&mut self) {
        self.error_text = None;
    }
}
