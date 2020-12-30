use std::io::Write;
use std::net::TcpStream;

use sr_lib::networking;

fn main() {
    let host = networking::get_server_host();
    let port = networking::get_server_port();
    let remote_addr = format!("{host}:{port}", host = host, port = port);

    match TcpStream::connect(remote_addr) {
        Ok(mut stream) => {
            stream.write(b"Hello!").unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
