use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use sr_lib::network::config::NetworkConfig;
use sr_lib::network::event::NetworkEvent;
use sr_lib::network::Network;
use termion::cursor::Goto;
use termion::event::Key;

use crate::input::Input;
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

        // TODO: replace with real update manager class, or mpsc?
        let mut updates: Vec<NetworkEvent> = Vec::new();

        'game: loop {
            let mut loops = 0;

            let _event = network.poll().expect("failed to poll");

            // TODO: serialize into same packet
            for update in updates.drain(..) {
                network
                    .send(update)
                    .expect("failed to send");
            }

            while clock.elapsed() > next_tick && loops < Game::MAX_FRAMESKIP {
                let input = self.input.recv().expect("input thread disconnected");
                match input {
                    Some(Key::Esc) => {
                        break 'game;
                    }
                    Some(e) => {
                        screen.handle_input(&e, &mut updates);
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
