use std::net::Ipv4Addr;

use enet::{
    Address, Error, Event, Host, Peer,
};

use crate::network::constants::{CONNECT_TIMEOUT_MS, FRONTEND_PORT, NUM_CHANNELS};

pub struct NetworkManager {
    host: Host<()>,
}

// TODO: maintain own connection instead of pub fn connect()
impl NetworkManager {
    pub fn new(host: Host<()>) -> NetworkManager {
        NetworkManager { host }
    }

    pub fn connect(&mut self, addr: Ipv4Addr) -> Result<Connection, Error> {
        self.host.connect(&Address::new(addr, FRONTEND_PORT), NUM_CHANNELS, 0)?;
        match self.host.service(CONNECT_TIMEOUT_MS)? {
            Some(Event::Connect(ref p)) => Ok(Connection { peer: p.clone() }),
            Some(_) => Err(Error(-1)),
            None => Err(Error(-2)),
        }
    }

    pub fn step(&mut self, timeout_ms: u32) -> Result<Option<Event<()>>, Error> {
        self.host.service(timeout_ms)
    }
}

pub struct Connection<'a> {
    pub peer: Peer<'a, ()>, // TODO: private
}
