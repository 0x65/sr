use sr_lib::network::config::ServerConfig;
use sr_lib::network::event::NetworkEvent;
use sr_lib::network::util::send_to_peer;
use sr_lib::network::Network;

mod db;

fn main() {
    // TODO: move to separate server startup script
    db::setup_db().expect("error during db setup");

    let mut network = Network::new()
        .expect("error during network setup")
        .create_server(ServerConfig::default())
        .expect("error during server setup");

    loop {
        if let Some(message) = network.poll().expect("failed to poll") {
            println!("Got packet contents: {:?}", message);
            match &message.event {
                NetworkEvent::LoginRequest(email) => {
                    let response = handle_login_request(&email);
                    send_to_peer(message.peer, response).expect("failed to send");
                }
                _ => {
                    println!("Got unsupported event type: {:?}", message);
                }
            };
        }
    }
}

fn handle_login_request(email: &str) -> NetworkEvent {
    match db::user::get_by_email(&email) {
        Ok(user) => NetworkEvent::LoginResponse(user.id),
        Err(_) => NetworkEvent::LoginResponse(0),
    }
}
