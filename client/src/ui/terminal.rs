use std::error::Error;
use std::io;

use termion::clear;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::terminal::Frame;
use tui::widgets::{Block, Borders};
use tui::Terminal;

use crate::ui::lib::events::{ClientEvent, ClientEvents};

pub type BackendT = TermionBackend<RawTerminal<io::Stdout>>;
pub type FrameT<'a> = Frame<'a, BackendT>;
pub type TerminalT = Terminal<BackendT>;

pub trait Screen {
    fn render(&self, frame: &mut FrameT);
    fn handle_event(&mut self, _event: &ClientEvent) {}
    fn cleanup(&self) {}
}

pub struct TerminalUI {
    terminal: TerminalT,
}

impl TerminalUI {
    pub fn initialize() -> Result<TerminalUI, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        println!("{}", clear::All);
        Ok(TerminalUI { terminal })
    }

    pub fn run(&mut self, mut screen: impl Screen) -> Result<(), Box<dyn Error>> {
        let events = ClientEvents::initialize();

        loop {
            // render UI
            self.terminal.draw(|f| {
                let border = Block::default().borders(Borders::ALL);
                f.render_widget(border, f.size());

                screen.render(f);
            })?;

            // process input
            let event = events.next()?;
            if is_exit_event(&event) {
                screen.cleanup();
                break;
            } else {
                screen.handle_event(&event);
            }
        }

        Ok(())
    }
}

fn is_exit_event(event: &ClientEvent) -> bool {
    match event {
        ClientEvent::Input(Key::Char('q')) => true,
        _ => false,
    }
}
