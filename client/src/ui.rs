use std::cmp;
use std::io;

use sr_lib::message::Message;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor};
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};
use tui::Terminal;

use crate::lib::layout::center_rect;

pub type BackendT = TermionBackend<RawTerminal<io::Stdout>>;
pub type FrameT<'a> = Frame<'a, BackendT>;
pub type TerminalT = Terminal<BackendT>;

pub trait Screen {
    fn render(&self, frame: &mut FrameT, game: Rect);
    fn handle_input(&mut self, _input: &Key, _updates: &mut Vec<Message>) {}
    fn handle_event(&mut self, _msg: &Message, _updates: &mut Vec<Message>) {}
    // TODO: more efficient implementation than Box
    fn transition(&self) -> Option<Box<dyn Screen>> {
        None
    }
}

pub struct UI {
    terminal: TerminalT,
}

impl UI {
    const MIN_WIDTH: u16 = 100;
    const MIN_HEIGHT: u16 = 40;

    const MAX_WIDTH: u16 = 160;
    const MAX_HEIGHT: u16 = 60;

    pub fn new() -> Result<UI, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        println!("{}", clear::All);
        Ok(UI { terminal })
    }

    pub fn render(&mut self, screen: &Box<dyn Screen>) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let size = f.size();
            if size.width < UI::MIN_WIDTH || size.height < UI::MIN_HEIGHT {
                let center_x = size.width / 2 - 10;
                let center_y = size.height / 2;
                println!(
                    "{}{}{}INCREASE SCREEN SIZE",
                    color::Bg(color::Red),
                    clear::All,
                    cursor::Goto(center_x, center_y)
                );
            } else {
                let game = center_rect(
                    cmp::min(size.width, UI::MAX_WIDTH),
                    cmp::min(size.height, UI::MAX_HEIGHT),
                    size,
                );
                let border = Block::default().borders(Borders::ALL);
                f.render_widget(border, game);
                screen.render(f, game);
            }
        })
    }
}
