use std::net::Ipv4Addr;

use enet::{
    Address, BandwidthLimit, ChannelLimit, Enet, Error, Event, Host as EHost, InitializationError,
    Peer,
};

const NUM_CHANNELS: usize = 32;
const CONNECT_TIMEOUT_MS: u32 = 1000;

fn get_host_addr(addr: Ipv4Addr) -> Address {
    Address::new(addr, 3333)
}

pub struct Network {
    raw: Enet,
}

impl Network {
    pub fn new() -> Result<Network, InitializationError> {
        let raw = Enet::new()?;
        Ok(Network { raw })
    }

    pub fn create_host(&self, local_addr: Option<&Ipv4Addr>) -> Result<Host, Error> {
        let raw = self.raw.create_host::<()>(
            local_addr.map(|a| get_host_addr(*a)).as_ref(),
            NUM_CHANNELS,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )?;
        Ok(Host { raw })
    }
}

pub struct Host {
    pub raw: EHost<()>, // TODO: private
}

impl Host {
    pub fn connect(&mut self, addr: Ipv4Addr) -> Result<Connection, Error> {
        self.raw.connect(&get_host_addr(addr), NUM_CHANNELS, 0)?;
        match self.raw.service(CONNECT_TIMEOUT_MS)? {
            Some(Event::Connect(ref p)) => Ok(Connection { raw: p.clone() }),
            Some(_) => Err(Error(-1)),
            None => Err(Error(-2)),
        }
    }
}

pub struct Connection<'a> {
    pub raw: Peer<'a, ()>, // TODO: private
}
