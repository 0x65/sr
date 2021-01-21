use sr_lib::message::Message;
use sr_lib::network::config::NetworkConfig;
use sr_lib::network::{Network, NetworkEvent};

mod db;

fn main() {
    // TODO: move to separate server startup script
    db::setup_db().expect("failed to set up DB");

    let mut network = Network::new(NetworkConfig::server()).expect("failed to init network");

    loop {
        match network.recv() {
            Ok(Some(network_event)) => match network_event {
                NetworkEvent::Message(msg, addr) => {
                    println!("Got {:?} from {:?}", msg, addr);
                    match msg {
                        Message::LoginRequest(email) => {
                            let response = handle_login_request(&email);
                            network
                                .send_to_peer(response, addr)
                                .expect("failed to send");
                        }
                        _ => {
                            println!("Got unsupported message type: {:?}", msg);
                        }
                    }
                }
                NetworkEvent::Connect(addr) => {
                    println!("Got connection from {:?}", addr);
                }
                NetworkEvent::Timeout(addr) => {
                    println!("Got timeout from {:?}", addr);
                }
                NetworkEvent::Disconnect(addr) => {
                    println!("Got disconnection from {:?}", addr);
                }
            },
            _ => { /* TODO: handle errors */ }
        }
    }
}

fn handle_login_request(email: &str) -> Message {
    match db::user::get_by_email(&email) {
        Ok(user) => Message::LoginResponse(user.id),
        Err(_) => Message::LoginResponse(0),
    }
}
