use std::str::from_utf8;

use enet::{Address, Error, Event, Host, Packet, PacketMode, Peer};

use crate::network::config::NetworkConfig;
use crate::network::constants::{CONNECT_TIMEOUT_MS, FRONTEND_PORT, NUM_CHANNELS};
use crate::network::message::NetworkMessage;

enum NetworkState {
    Unconnected,
    Connecting,
    Ready,
}

pub struct NetworkManager {
    config: NetworkConfig,
    host: Host<()>,
    state: NetworkState,
}

// TODO: handle unexpected events (e.g. receive on connecting)
// TODO: fix unwraps
// TODO: close connection (make sure can read disconnect msg on server on 'q')
// TODO: logging
impl NetworkManager {
    pub fn new(config: NetworkConfig, host: Host<()>) -> NetworkManager {
        let state = NetworkManager::init_state(&config);
        NetworkManager {
            config,
            host,
            state,
        }
    }

    pub fn poll(&mut self) -> Result<Option<NetworkMessage>, Error> {
        match self.state {
            NetworkState::Unconnected => {
                self.connect()?;
                Ok(None)
            }
            NetworkState::Connecting => self.service(CONNECT_TIMEOUT_MS),
            NetworkState::Ready => self.service(0),
        }
    }

    pub fn send(&mut self, message: NetworkMessage) -> Result<(), Error> {
        match self.remote() {
            Some(mut peer) => {
                let data = message.data.as_bytes();
                let packet = Packet::new(data, PacketMode::ReliableSequenced)?;
                peer.send_packet(packet, 0)
            }
            None => Err(Error(-1)),
        }
    }

    fn init_state(config: &NetworkConfig) -> NetworkState {
        if config.remote_addr.is_some() {
            NetworkState::Unconnected
        } else {
            NetworkState::Ready
        }
    }

    fn connect(&mut self) -> Result<(), Error> {
        match self.config.remote_addr {
            Some(addr) => {
                self.state = NetworkState::Connecting;
                let remote_addr = Address::new(addr, FRONTEND_PORT);
                self.host.connect(&remote_addr, NUM_CHANNELS, 0)?;
            }
            None => {
                // should never really happen based on init_state()
                self.state = NetworkState::Ready;
            }
        }
        Ok(())
    }

    fn remote(&mut self) -> Option<Peer<()>> {
        match self.state {
            // based on NUM_CHANNELS = 1
            NetworkState::Ready => self.host.peers().next(),
            _ => None,
        }
    }

    fn service(&mut self, timeout_ms: u32) -> Result<Option<NetworkMessage>, Error> {
        match self.host.service(timeout_ms)? {
            Some(Event::Receive { ref packet, .. }) => {
                let data = from_utf8(packet.data()).unwrap().to_string();
                Ok(Some(NetworkMessage { data }))
            }
            Some(Event::Connect(_)) => {
                self.state = NetworkState::Ready;
                Ok(None)
            }
            Some(Event::Disconnect(_, _)) => {
                self.state = NetworkManager::init_state(&self.config);
                Ok(None)
            }
            None => Ok(None),
        }
    }
}
