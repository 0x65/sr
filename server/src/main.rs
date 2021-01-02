use std::net::Ipv4Addr;
use std::str::from_utf8;

use enet::{Event, Packet, PacketMode};

use sr_lib::networking::Network;

mod db;

fn main() {
    // TODO: move to separate server startup script
    db::setup_db().expect("error during db setup");

    let network = Network::new().expect("error during network setup");
    let mut host = network
        .create_host(Some(&Ipv4Addr::UNSPECIFIED))
        .expect("error during host creation");

    loop {
        // TODO: timeout of 0?
        match host.raw.service(1000).expect("service failed") {
            Some(Event::Connect(_)) => println!("new connection!"),
            Some(Event::Disconnect(_, _)) => println!("disconnect!"),
            Some(Event::Receive {
                ref mut sender,
                channel_id,
                ref packet,
            }) => {
                let content = from_utf8(packet.data()).unwrap();
                println!(
                    "got packet from {:?} on channel {}, content: {}",
                    sender, channel_id, content
                );
                sender
                    .send_packet(
                        Packet::new(b"supbro", PacketMode::ReliableSequenced).unwrap(),
                        1,
                    )
                    .unwrap();
            }
            _ => (),
        }
    }
}
