use termion::event::Key;
use tui::layout::{Constraint, Direction, Layout};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use crate::ui::lib::events::ClientEvent;
use crate::ui::lib::layout::center_rect;
use crate::ui::terminal::{FrameT, Screen};

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

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(1)].as_ref())
            .split(dialog_bounds);

        let email_text = Spans::from(vec![Span::raw("Email: "), Span::raw(&self.email)]);

        frame.render_widget(Paragraph::new(email_text), chunks[0]);
        frame.set_cursor(chunks[0].x + self.email.len() as u16 + 7, chunks[0].y);
    }

    fn handle_event(&mut self, event: &ClientEvent) {
        match event {
            ClientEvent::Input(key) => {
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
