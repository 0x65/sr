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
    fn render(&self, frame: &mut FrameT, game: Rect) {
        let bounds = center_rect(LoadingScreen::WIDTH, LoadingScreen::HEIGHT, game);

        // main dialog
        frame.render_widget(Block::default().borders(Borders::ALL), bounds);

        // loading text
        frame.render_widget(
            Paragraph::new(Span::raw("L O A D I N G")),
            Rect {
                x: bounds.x + 8,
                y: bounds.y + 2,
                width: 13,
                height: 1,
            },
        );
    }
}
