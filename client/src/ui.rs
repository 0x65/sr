use std::io;

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
    fn render(&self, frame: &mut FrameT, interp_ms: f64);
    fn handle_input(&mut self, _input: &Key, _events: &mut Vec<String>) {}
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

    pub fn render(&mut self, screen: &impl Screen, interpolation: f64) -> Result<(), io::Error> {
        self.terminal.draw(|f| {
            let border = Block::default().borders(Borders::ALL);
            f.render_widget(border, f.size());

            screen.render(f, interpolation);
        })
    }
}
