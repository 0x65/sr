use sr_lib::network::config::NetworkConfig;
use sr_lib::network::Network;

use crate::game::Game;
use crate::input::Input;
use crate::ui::UI;

mod game;
mod input;
mod lib;
mod screens;
mod ui;

fn main() {
    let network_config = NetworkConfig::client();

    Game::new(
        Input::new(),
        Network::new(network_config).expect("failed to init network"),
        UI::new().expect("failed to init UI"),
    )
    .run()
}
