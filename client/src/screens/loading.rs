use tui::layout::Rect;
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph};

use crate::lib::layout::center_rect;
use crate::ui::{FrameT, Screen};

#[derive(PartialEq, Debug)]
enum LoadingState {
    Loading,
    Finished,
}

// TODO: generalize this for other kinds of loading (pass in a FnOnce?)
pub struct LoadingScreen {
    user_id: i64,
}

impl LoadingScreen {
    const WIDTH: u16 = 30;
    const HEIGHT: u16 = 5;

    pub fn new(user_id: i64) -> LoadingScreen {
        LoadingScreen { user_id }
    }
}

impl Screen for LoadingScreen {
    fn render(&self, frame: &mut FrameT) {
        // TODO: instantiate some of these elements statically
        let bounds = center_rect(LoadingScreen::WIDTH, LoadingScreen::HEIGHT, frame.size());

        let dialog = Block::default().borders(Borders::ALL);

        let label = Paragraph::new(Span::raw("L O A D I N G"));

        frame.render_widget(dialog, bounds);

        frame.render_widget(
            label,
            Rect {
                x: bounds.x + 8,
                y: bounds.y + 2,
                width: 13,
                height: 1,
            },
        );
    }
}
