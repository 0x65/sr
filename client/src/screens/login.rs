use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph};

use crate::input::InputEvent;
use crate::lib::layout::center_rect;
use crate::ui::{FrameT, Screen};

pub struct LoginScreen {
    email: String,
}

impl LoginScreen {
    pub fn new() -> LoginScreen {
        LoginScreen {
            email: String::with_capacity(8),
        }
    }
}

impl Screen for LoginScreen {
    fn render(&self, frame: &mut FrameT) {
        let dialog_bounds = center_rect(60, 20, frame.size());

        let dialog = Block::default().title("Login").borders(Borders::ALL);

        frame.render_widget(dialog, dialog_bounds);

        let vchunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(10)].as_ref())
            .split(dialog_bounds);

        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Length(10), Constraint::Min(1)].as_ref())
            .split(vchunks[0]);

        frame.render_widget(Paragraph::new(Span::raw("Email: ")), hchunks[0]);

        let email_box =
            Paragraph::new(Span::raw(&self.email)).block(Block::default().borders(Borders::ALL));

        let email_rect = Rect {
            x: hchunks[1].x,
            y: hchunks[1].y - 1,
            width: 12, // 8 (email) + 2 (border) + 2 (margin)
            height: 3, // 1 (email) + 2 (border)
        };

        frame.render_widget(email_box, email_rect);

        frame.set_cursor(hchunks[1].x + self.email.len() as u16 + 1, hchunks[1].y);
    }

    fn handle_input(&mut self, input: &InputEvent) {
        match input {
            InputEvent::Input(key) => {
                process_key_press(key, &mut self.email);
            }
        }
    }
}

fn process_key_press(key: &Key, buffer: &mut String) {
    match key {
        Key::Char(ch) => {
            if buffer.len() < buffer.capacity() {
                buffer.push(*ch);
            }
        }
        Key::Backspace => {
            buffer.pop();
        }
        _ => {}
    }
}
