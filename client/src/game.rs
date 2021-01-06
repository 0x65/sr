use std::time::{Duration, Instant};

use sr_lib::networking::Network;
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

        let clock = Instant::now();
        let mut next_tick = clock.elapsed();

        let mut debug_clock = Instant::now();
        let mut debug_num_frames = 0;

        'game: loop {
            let mut loops = 0;
            while clock.elapsed() > next_tick && loops < Game::MAX_FRAMESKIP {
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
