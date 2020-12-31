use tui::layout::Rect;
use tui::widgets::{Block, Borders};

use crate::ui::lib::layout::center_rect;
use crate::ui::terminal::{FrameT, Screen};

pub struct LoginScreen {}

impl Screen for LoginScreen {
    fn render(&self, frame: &mut FrameT) {
        let dialog_bounds = Rect {
            x: 0,
            y: 0,
            width: 40,
            height: 10,
        };
        let centered_bounds = center_rect(dialog_bounds, frame.size());

        let dialog = Block::default().title("Dialog").borders(Borders::ALL);
        frame.render_widget(dialog, centered_bounds);
    }
}
