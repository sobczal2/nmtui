use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};
use tui_big_text::{BigText, PixelSize};

pub fn render_header(frame: &mut Frame, area: Rect) {
    let block = Block::new().padding(Padding::uniform(1));
    let label = BigText::builder()
        .pixel_size(PixelSize::Full)
        .lines(vec!["NMVI".fg(Color::Rgb(255, 204, 0)).into()])
        .alignment(Alignment::Center)
        .build();

    frame.render_widget(label, block.inner(area));
}
