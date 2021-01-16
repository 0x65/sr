use std::time::{Duration, Instant};

use sr_lib::network::config::ClientConfig;
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
        // TODO: more efficient implementation than Box
        let mut screen: Box<dyn Screen> = Box::new(LoginScreen::new());

        // TODO: just take a NetworkManager in constructor?
        let mut network = self
            .network
            .create_client(ClientConfig::default())
            .expect("error during client setup");

        let clock = Instant::now();
        let mut next_tick = clock.elapsed();

        let mut debug_clock = Instant::now();
        let mut debug_num_frames = 0;

        // TODO: replace with real update manager class, or mpsc?
        let mut updates: Vec<NetworkEvent> = Vec::new();

        'game: loop {
            let mut loops = 0;

            while clock.elapsed() > next_tick && loops < Game::MAX_FRAMESKIP {
                // TODO: move network polling/sending to separate thread
                // (serialization, deserialization, etc. is not necessary for critical path)
                if let Some(ref e) = network.poll().expect("failed to poll") {
                    screen.handle_event(&e.event, &mut updates);
                }

                if let Some(key) = self.input.recv().expect("input thread disconnected") {
                    if Game::exit(key) {
                        break 'game;
                    } else {
                        screen.handle_input(&key, &mut updates);
                    }
                }

                // TODO: serialize into same packet
                for update in updates.drain(..) {
                    network.send(update).expect("failed to send");
                }

                next_tick += Game::SKIP_TICKS;
                loops += 1;
            }

            /*
            let interp_ms = ((clock.elapsed() + Game::SKIP_TICKS - next_tick).as_millis() as f64)
                / (Game::SKIP_TICKS.as_millis() as f64);
            */

            self.ui.render(&screen).expect("failed to render screen");

            Game::display_debug_info(&mut debug_clock, &mut debug_num_frames);

            if let Some(new_screen) = screen.transition() {
                screen = new_screen;
            }
        }
    }

    fn exit(key: Key) -> bool {
        key == Key::Esc
    }

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
