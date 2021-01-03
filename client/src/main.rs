use sr_lib::networking::Network;

use crate::game::Game;
use crate::input::Input;
use crate::ui::UI;

mod game;
mod input;
mod lib;
mod screens;
mod ui;

fn main() {
    Game::new(
        Input::new(),
        Network::new().expect("failed to init network"),
        UI::new().expect("failed to init UI"),
    )
    .run()
}
