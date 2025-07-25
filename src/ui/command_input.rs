use ratatui::{prelude::*, widgets::Block};

pub fn render_command_input(text: impl AsRef<str>, frame: &mut Frame, area: Rect) {
    let cmd_block = Block::bordered().border_style(Style::new().fg(Color::Rgb(235, 91, 0)));
    frame.render_widget(cmd_block.clone(), area);

    let cmd = Line::from(text.as_ref());
    frame.render_widget(cmd, cmd_block.inner(area));
}
