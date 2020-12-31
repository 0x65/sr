use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn center_rect(width: u16, height: u16, area: Rect) -> Rect {
    let margin_y = (area.height - height) / 2;
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(margin_y),
                Constraint::Length(height),
                Constraint::Length(margin_y),
            ]
            .as_ref(),
        )
        .split(area)[1];

    let margin_x = (area.width - width) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(margin_x),
                Constraint::Length(width),
                Constraint::Length(margin_x),
            ]
            .as_ref(),
        )
        .split(vertical)[1]
}
