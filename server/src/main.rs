use std::io::Read;
use std::net::{Shutdown, TcpListener};
use std::str::from_utf8;
use std::thread;

use sr_lib::networking;

mod db;

fn main() {
    // TODO: move to separate server startup script
    match db::setup_db() {
        Ok(_) => {
            eprintln!("Successfully set up DB...");
        }
        Err(e) => {
            eprintln!("Fatal error during setup of DB: {}", e);
            panic!()
        }
    };

    let port = networking::get_server_port();
    let local_addr = format!("0.0.0.0:{port}", port = port);
    let listener = TcpListener::bind(local_addr).unwrap();

    println!("Listening on port {port}...", port = port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("New connection from {}!", peer_addr);

                thread::spawn(move || {
                    let mut buffer = [0 as u8; 64];
                    while match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size > 0 {
                                println!("Received data: {}", from_utf8(&buffer[0..size]).unwrap());
                            }
                            true
                        }
                        Err(e) => {
                            println!("Encountered read error: {}", e);
                            stream.shutdown(Shutdown::Both).unwrap();
                            false
                        }
                    } {}
                });
            }
            Err(e) => {
                println!("Encountered socket error: {}", e);
            }
        }
    }

    drop(listener);
}
