use sr_lib::networking::Network;

use crate::events::Events;
use crate::game::Game;
use crate::ui::UI;

mod events;
mod game;
mod lib;
mod screens;
mod ui;

fn main() {
    Game::new(
        Events::new(),
        Network::new().expect("failed to init network"),
        UI::new().expect("failed to init UI"),
    )
    .run()
}
