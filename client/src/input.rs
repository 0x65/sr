use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub enum InputEvent {
    Input(Key),
}

#[derive(Debug)]
pub enum InputError {
    Disconnected,
}

pub struct Input {
    receiver: mpsc::Receiver<InputEvent>,
    handle: thread::JoinHandle<()>,
}

impl Input {
    pub fn new() -> Input {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            let stdin = io::stdin();
            for event in stdin.keys() {
                if let Ok(key) = event {
                    if let Err(err) = sender.send(InputEvent::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });
        Input { receiver, handle }
    }

    pub fn recv(&self) -> Result<Option<InputEvent>, InputError> {
        match self.receiver.try_recv() {
            Ok(e) => Ok(Some(e)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(InputError::Disconnected),
        }
    }
}
