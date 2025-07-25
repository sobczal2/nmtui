use ratatui::{prelude::*, symbols::border, widgets::Block};

use crate::state::Mode;

pub fn render_main_block(frame: &mut Frame, mode: Mode, area: Rect) {
    let mode = Line::from(mode.to_string());
    let block = Block::bordered()
        .title_bottom(mode.left_aligned())
        .border_set(border::THICK)
        .fg(Color::White);
    frame.render_widget(block, area);
}
