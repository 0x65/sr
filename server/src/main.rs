use std::net::Ipv4Addr;

use sr_lib::network::config::NetworkConfig;
use sr_lib::network::event::NetworkEvent;
use sr_lib::network::manager::NetworkManager;
use sr_lib::network::Network;

mod db;

fn main() {
    // TODO: move to separate server startup script
    db::setup_db().expect("error during db setup");

    let network_config = NetworkConfig::default(Some(Ipv4Addr::UNSPECIFIED), None);
    let mut network = Network::new()
        .expect("error during network setup")
        .create_manager(network_config)
        .expect("error during host creation");

    loop {
        if let Some(message) = network.poll().expect("failed to poll") {
            println!("Got packet contents: {:?}", message);
            match message.event {
                NetworkEvent::LoginRequest(email) => {
                    let response = handle_login_request(&email);
                    NetworkManager::send_to_peer(response, message.peer).expect("failed to respond")
                }
                _ => {
                    println!("Got unsupported event type: {:?}", message);
                }
            }
        }
    }
}

fn handle_login_request(email: &str) -> NetworkEvent {
    match db::user::get_by_email(&email) {
        Ok(user) => NetworkEvent::LoginResponse(user.id),
        Err(_) => NetworkEvent::LoginResponse(0),
    }
}
