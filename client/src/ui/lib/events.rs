use std::io;
use std::sync::mpsc;
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

pub enum ClientEvent {
    Input(Key),
}

pub struct ClientEvents {
    receiver: mpsc::Receiver<ClientEvent>,
    input_handle: thread::JoinHandle<()>,
}

impl ClientEvents {
    pub fn initialize() -> ClientEvents {
        // if adding additional handles: re-use receiver, clone sender
        let (sender, receiver) = mpsc::channel();

        let input_handle = thread::spawn(move || {
            let stdin = io::stdin();
            for event in stdin.keys() {
                if let Ok(key) = event {
                    if let Err(err) = sender.send(ClientEvent::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                }
            }
        });

        ClientEvents {
            receiver,
            input_handle,
        }
    }

    pub fn next(&self) -> Result<ClientEvent, mpsc::RecvError> {
        self.receiver.recv()
    }
}
