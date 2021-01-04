use std::time::Instant;

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
    const TICKS_PER_SECOND: u32 = 25;
    const SKIP_TICKS: u32 = 1000 / Game::TICKS_PER_SECOND;
    const MAX_FRAMESKIP: u32 = 5;

    pub fn new(input: Input, network: Network, ui: UI) -> Game {
        Game { input, network, ui }
    }

    // TODO: proper error handling
    pub fn run(&mut self) {
        let mut screen = LoginScreen::new();

        let clock = Instant::now();
        let mut next_tick = clock.elapsed().as_millis();

        // TODO: replace int additions/as_millis conversions with Duration?
        'game: loop {
            let mut loops = 0;
            while clock.elapsed().as_millis() > next_tick && loops < Game::MAX_FRAMESKIP {
                let input = self.input.recv().expect("input thread disconnected");
                match input {
                    Some(InputEvent::Input(Key::Char('q'))) => {
                        break 'game;
                    }
                    Some(e) => {
                        screen.handle_input(&e);
                    }
                    None => {}
                }

                next_tick += Game::SKIP_TICKS as u128;
                loops += 1;
            }

            let interpolation = (clock.elapsed().as_millis() as f64 + Game::SKIP_TICKS as f64
                - next_tick as f64)
                / Game::SKIP_TICKS as f64;

            self.ui
                .render(&screen, interpolation)
                .expect("failed to render screen");
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
