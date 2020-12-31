use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn center_rect(rect: Rect, area: Rect) -> Rect {
    let margin_y = (area.height - rect.height) / 2;
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(margin_y),
                Constraint::Length(rect.height),
                Constraint::Length(margin_y),
            ]
            .as_ref(),
        )
        .split(area)[1];

    let margin_x = (area.width - rect.width) / 2;
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(margin_x),
                Constraint::Length(rect.width),
                Constraint::Length(margin_x),
            ]
            .as_ref(),
        )
        .split(vertical)[1]
}
