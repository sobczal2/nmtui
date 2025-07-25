use std::rc::Rc;

use ratatui::prelude::*;

pub fn main_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(10),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area)
}
