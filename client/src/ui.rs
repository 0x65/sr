use std::io;

use sr_lib::network::event::NetworkEvent;
use termion::clear;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub type BackendT = TermionBackend<RawTerminal<io::Stdout>>;
pub type FrameT<'a> = Frame<'a, BackendT>;
pub type TerminalT = Terminal<BackendT>;

pub trait Screen {
    fn render(&self, frame: &mut FrameT);
    fn handle_input(&mut self, _input: &Key, _updates: &mut Vec<NetworkEvent>) {}
    fn handle_event(&mut self, _event: &NetworkEvent, _updates: &mut Vec<NetworkEvent>) {}
    // TODO: more efficient implementation than Box
    fn transition(&self) -> Option<Box<dyn Screen>> {
        None
    }
}

pub struct UI {
    terminal: TerminalT,
}

impl UI {
    pub fn new() -> Result<UI, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        println!("{}", clear::All);
        Ok(UI { terminal })
    }

    pub fn render(&mut self, screen: &Box<dyn Screen>) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let border = Block::default().borders(Borders::ALL);
            f.render_widget(border, f.size());
            screen.render(f);
        })
    }
}
