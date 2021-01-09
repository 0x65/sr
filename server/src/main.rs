use std::net::Ipv4Addr;

use sr_lib::network::config::NetworkConfig;
use sr_lib::network::Network;

mod db;

fn main() {
    // TODO: move to separate server startup script
    db::setup_db().expect("error during db setup");

    let mut network = Network::new()
        .expect("error during network setup")
        .create_manager(NetworkConfig {
            local_addr: Some(Ipv4Addr::UNSPECIFIED),
            remote_addr: None,
        })
        .expect("error during host creation");

    loop {
        let poll = network.poll().expect("failed to poll");

        if poll.data.is_some() {
            println!("got packet content: {:?}", poll.data);
        }
    }
}
