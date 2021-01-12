use sr_lib::network::event::NetworkEvent;
use termion::event::Key;
use tui::layout::Rect;
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph};

use crate::lib::layout::center_rect;
use crate::ui::{FrameT, Screen};

#[derive(PartialEq, Debug)]
enum LoginScreenState {
    Initial,
    Processing,
    Success,
    Error,
}

pub struct LoginScreen {
    email: String,
    state: LoginScreenState,
}

impl LoginScreen {
    const WIDTH: u16 = 60;
    const HEIGHT: u16 = 20;

    pub fn new() -> LoginScreen {
        LoginScreen {
            email: String::with_capacity(8),
            state: LoginScreenState::Initial,
        }
    }
}

impl Screen for LoginScreen {
    fn render(&self, frame: &mut FrameT, _interp_ms: f64) {
        let bounds = center_rect(LoginScreen::WIDTH, LoginScreen::HEIGHT, frame.size());

        let dialog = Block::default().title("Login").borders(Borders::ALL);

        let email_label = Paragraph::new(Span::raw("Email: "));

        let email_input =
            Paragraph::new(Span::raw(&self.email)).block(Block::default().borders(Borders::ALL));

        let instruction_text = match self.state {
            LoginScreenState::Initial => "Press <ENTER> to submit.",
            LoginScreenState::Processing => "Processing...",
            LoginScreenState::Success => "SUCCESS!",
            LoginScreenState::Error => "ERROR! Invalid credentials.",
        };

        let instruction_label = Paragraph::new(Span::raw(instruction_text));

        frame.render_widget(dialog, bounds);

        frame.render_widget(
            email_label,
            Rect {
                x: bounds.x + 3,
                y: bounds.y + 3,
                width: 10,
                height: 1,
            },
        );

        frame.render_widget(
            email_input,
            Rect {
                x: bounds.x + 13,
                y: bounds.y + 2,
                width: 12,
                height: 3,
            },
        );

        frame.render_widget(
            instruction_label,
            Rect {
                x: bounds.x + 3,
                y: bounds.y + 5,
                width: instruction_text.len() as u16,
                height: 1,
            },
        );

        frame.set_cursor(bounds.x + 14 + self.email.len() as u16, bounds.y + 3);
    }

    fn handle_input(&mut self, input: &Key, updates: &mut Vec<NetworkEvent>) {
        if self.state != LoginScreenState::Processing {
            match input {
                Key::Char('\n') => {
                    if !self.email.is_empty() {
                        updates.push(NetworkEvent::LoginRequest(self.email.clone()));
                        self.state = LoginScreenState::Processing;
                    }
                }
                Key::Char(ch) => {
                    if (ch.is_ascii_alphanumeric() || *ch == '@' || *ch == '.')
                        && self.email.len() < self.email.capacity()
                    {
                        self.email.push(*ch);
                    }
                }
                Key::Backspace | Key::Delete => {
                    self.email.pop();
                }
                _ => {}
            }
        }
    }

    fn handle_event(&mut self, event: &NetworkEvent, _updates: &mut Vec<NetworkEvent>) {
        match event {
            NetworkEvent::LoginResponse(user_id) => {
                if self.state == LoginScreenState::Processing {
                    if *user_id > 0 {
                        self.state = LoginScreenState::Success;
                    } else {
                        self.state = LoginScreenState::Error;
                    }
                }
            }
            _ => { /* ignore other events */ }
        }
    }
}
