use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub enum Event {
    Input(Key),
}

#[derive(Debug)]
pub enum EventError {
    Disconnected,
}

pub struct Events {
    receiver: mpsc::Receiver<Event>,
    input_handle: thread::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Events {
        // if adding additional handles: re-use receiver, clone sender
        let (sender, receiver) = mpsc::channel();

        let input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for event in stdin.keys() {
                if let Ok(key) = event {
                    if let Err(err) = sender.send(Event::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });

        Events {
            receiver,
            input_handle,
        }
    }

    pub fn recv(&self) -> Result<Option<Event>, EventError> {
        match self.receiver.try_recv() {
            Ok(e) => Ok(Some(e)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(EventError::Disconnected),
        }
    }
}
