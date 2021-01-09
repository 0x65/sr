use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use sr_lib::network::config::NetworkConfig;
use sr_lib::network::Network;
use termion::cursor::Goto;
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
    const TICKS_PER_SECOND: u64 = 25;
    const SKIP_TICKS: Duration = Duration::from_millis(1000 / Game::TICKS_PER_SECOND);
    const MAX_FRAMESKIP: u32 = 5;

    pub fn new(input: Input, network: Network, ui: UI) -> Game {
        Game { input, network, ui }
    }

    // TODO: proper error handling
    pub fn run(&mut self) {
        let mut screen = LoginScreen::new();

        // TODO: just take a NetworkManager in constructor?
        let mut network = self
            .network
            .create_manager(NetworkConfig {
                local_addr: None,
                remote_addr: Some(Ipv4Addr::LOCALHOST),
            })
            .expect("failed to create manager");

        let clock = Instant::now();
        let mut next_tick = clock.elapsed();

        let mut debug_clock = Instant::now();
        let mut debug_num_frames = 0;

        let mut message: Option<&str> = None;

        'game: loop {
            let mut loops = 0;

            //////////////////////////
            use enet::{Packet, PacketMode};
            let _data = network.poll().expect("failed to poll");
            let msg = message.take();
            match msg {
                Some(x) => {
                    let ref mut peer = network.remote().unwrap();
                    let packet = Packet::new(x.as_bytes(), PacketMode::ReliableSequenced)
                        .expect("failed to create");
                    peer.send_packet(packet, 0).expect("failed to send");
                }
                None => {}
            }
            //////////////////////////

            while clock.elapsed() > next_tick && loops < Game::MAX_FRAMESKIP {
                let input = self.input.recv().expect("input thread disconnected");
                match input {
                    //////////////////
                    Some(InputEvent::Input(Key::Char('a'))) => {
                        message.replace("pressed A lol");
                    }
                    //////////////////
                    Some(InputEvent::Input(Key::Char('q'))) => {
                        break 'game;
                    }
                    Some(e) => {
                        screen.handle_input(&e);
                    }
                    None => {}
                }

                next_tick += Game::SKIP_TICKS;
                loops += 1;
            }

            let interp_ms = ((clock.elapsed() + Game::SKIP_TICKS - next_tick).as_millis() as f64)
                / (Game::SKIP_TICKS.as_millis() as f64);

            self.ui
                .render(&screen, interp_ms)
                .expect("failed to render screen");

            Game::display_debug_info(&mut debug_clock, &mut debug_num_frames);
        }
    }

    // TODO: make separate debug info screen
    fn display_debug_info(clock: &mut Instant, num_frames: &mut usize) {
        *num_frames += 1;
        let elapsed = clock.elapsed().as_millis();
        if elapsed > 1000 {
            println!("{}FPS: {}", Goto(2, 2), *num_frames);
            *clock = Instant::now();
            *num_frames = 0;
        }
    }
}
