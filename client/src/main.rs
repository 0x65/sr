use std::net::Ipv4Addr;

use enet::{Packet, PacketMode};

use sr_lib::networking::Network;

/*
use crate::ui::screens::login::LoginScreen;
use crate::ui::terminal::TerminalUI;
*/

mod ui;

fn main() {
    let network = Network::new().expect("error during network setup");
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

    /*
    let mut tui = TerminalUI::initialize().expect("failed to init terminal");
    tui.run(LoginScreen::new()).expect("i/o error");
    */
}
