use sr_lib::message::Message;
use termion::event::Key;
use tui::layout::Rect;
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph};

use crate::lib::layout::center_rect;
use crate::screens::loading::LoadingScreen;
use crate::ui::{FrameT, Screen};

#[derive(PartialEq, Debug)]
enum LoginState {
    Initial,
    Processing,
    Error,
    Success(i64),
}

pub struct LoginScreen {
    email: String,
    state: LoginState,
}

impl LoginScreen {
    const WIDTH: u16 = 60;
    const HEIGHT: u16 = 20;
    const EMAIL_MAX_LEN: usize = 8;

    pub fn new() -> LoginScreen {
        LoginScreen {
            email: String::with_capacity(LoginScreen::EMAIL_MAX_LEN),
            state: LoginState::Initial,
        }
    }
}

impl Screen for LoginScreen {
    fn render(&self, frame: &mut FrameT, game: Rect) {
        let bounds = center_rect(LoginScreen::WIDTH, LoginScreen::HEIGHT, game);

        let instruction = match self.state {
            LoginState::Initial => "Press <ENTER> to submit.",
            LoginState::Processing => "Processing...",
            LoginState::Success(_) => "SUCCESS!",
            LoginState::Error => "ERROR! Invalid credentials.",
        };

        // main dialog
        frame.render_widget(
            Block::default().title("Login").borders(Borders::ALL),
            bounds,
        );

        // email label
        frame.render_widget(
            Paragraph::new(Span::raw("Email: ")),
            Rect {
                x: bounds.x + 3,
                y: bounds.y + 3,
                width: 10,
                height: 1,
            },
        );

        // email input
        frame.render_widget(
            Paragraph::new(Span::raw(&self.email)).block(Block::default().borders(Borders::ALL)),
            Rect {
                x: bounds.x + 13,
                y: bounds.y + 2,
                width: 12,
                height: 3,
            },
        );

        // instruction text
        frame.render_widget(
            Paragraph::new(Span::raw(instruction)),
            Rect {
                x: bounds.x + 3,
                y: bounds.y + 6,
                width: instruction.len() as u16,
                height: 1,
            },
        );

        // set input cursor based on offsets above
        frame.set_cursor(bounds.x + 14 + self.email.len() as u16, bounds.y + 3);
    }

    fn handle_input(&mut self, input: &Key, updates: &mut Vec<Message>) {
        if self.state != LoginState::Processing {
            match input {
                Key::Char('\n') => {
                    if !self.email.is_empty() {
                        updates.push(Message::LoginRequest(self.email.clone()));
                        self.state = LoginState::Processing;
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

    fn handle_event(&mut self, msg: &Message, _updates: &mut Vec<Message>) {
        match msg {
            Message::LoginResponse(user_id) => {
                if self.state == LoginState::Processing {
                    if *user_id > 0 {
                        self.state = LoginState::Success(*user_id);
                    } else {
                        self.state = LoginState::Error;
                    }
                }
            }
            _ => { /* ignore other events */ }
        }
    }

    fn transition(&self) -> Option<Box<dyn Screen>> {
        match self.state {
            LoginState::Success(user_id) => Some(Box::new(LoadingScreen::new(user_id))),
            _ => None,
        }
    }
}
