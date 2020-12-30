use std::net::TcpStream;

use sr_lib::commands::ClientCommand;
use sr_lib::networking;

mod ui;

struct ClientSession {
    session_id: u64,
    connection: Option<TcpStream>,
}

impl ClientSession {
    fn initialize(session_id: u64) -> ClientSession {
        ClientSession {
            session_id: session_id,
            connection: None,
        }
    }

    fn send_cmd(&mut self, cmd: &ClientCommand) -> bool {
        if self.connection.is_none() {
            self.connection = self._open_connection();
        }

        match &self.connection {
            Some(connection) => {
                match serde_json::to_writer(connection, &cmd) {
                    Ok(_) => true,
                    Err(e) => {
                        let serialized = serde_json::to_string(&cmd).unwrap();
                        eprintln!("SEND ERROR: Dropping command {} ({})", serialized, e);
                        // TODO: only reset if this is a connection error
                        self.connection = None;
                        false
                    }
                }
            }
            None => {
                let serialized = serde_json::to_string(&cmd).unwrap();
                eprintln!("NO CONNECTION: Dropping command {}", serialized);
                false
            }
        }
    }

    fn _open_connection(&mut self) -> Option<TcpStream> {
        let host = networking::get_server_host();
        let port = networking::get_server_port();
        let remote_addr = format!("{host}:{port}", host = host, port = port);

        match TcpStream::connect(&remote_addr) {
            Ok(connection) => {
                eprintln!("Opened remote connection to {}", remote_addr);
                Some(connection)
            }
            Err(e) => {
                eprintln!("Could not open remote connection: {}", e);
                None
            }
        }
    }
}

fn main() {
    let screen = ui::Screen::initialize();
    screen.run();

    /*
    let mut session = ClientSession::initialize(1);
    let cmd = ClientCommand::HEARTBEAT;
    session.send_cmd(&cmd);
    */
}
