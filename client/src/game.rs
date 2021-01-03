use sr_lib::networking::Network;
use termion::event::Key;

use crate::input::{Input, InputEvent};
use crate::screens::login::LoginScreen;
use crate::ui::{Screen, UI};

pub struct Game {
    input: Input,
    network: Network,
    ui: UI,
}

impl Game {
    pub fn new(input: Input, network: Network, ui: UI) -> Game {
        Game { input, network, ui }
    }

    pub fn run(&mut self) {
        let mut screen = LoginScreen::new();

        loop {
            // TODO: proper error handling
            self.ui.render(&screen).expect("failed to render screen");

            let input = self.input.recv().expect("input thread disconnected");

            match input {
                Some(InputEvent::Input(Key::Char('q'))) => {
                    break;
                }
                Some(e) => {
                    screen.handle_input(&e);
                }
                None => {}
            }
        }
    }
}

/*
use std::net::Ipv4Addr;

use enet::{Packet, PacketMode};

    let mut host = network
        .create_host(None)
        .expect("error during host creation");

    let mut peer = host
        .connect(Ipv4Addr::LOCALHOST)
        .expect("error during connect");

    peer.raw
        .send_packet(
            Packet::new(b"harro", PacketMode::ReliableSequenced).unwrap(),
            1,
        )
        .unwrap();

    // peer.raw.disconnect_later(5);

    loop {
        let e = host.raw.service(1000).unwrap();
        println!("received event: {:#?}", e);
    }
*/
