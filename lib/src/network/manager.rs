use enet::{Address, Error, Event, Host, Packet, PacketMode, Peer};

use crate::network::config::NetworkConfig;
use crate::network::constants::SERVER_FRONTEND_PORT;
use crate::network::event::{deserialize, serialize, NetworkEvent};

enum NetworkState {
    Unconnected,
    Connecting,
    Ready,
}

#[derive(Debug)]
pub struct NetworkResponse<'a> {
    pub event: NetworkEvent,
    pub peer: Peer<'a, ()>,
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

    pub fn poll(&mut self) -> Result<Option<NetworkResponse>, Error> {
        match self.state {
            NetworkState::Unconnected => {
                self.connect()?;
                Ok(None)
            }
            NetworkState::Connecting => self.service(0),
            NetworkState::Ready => self.service(0),
        }
    }

    // TODO: separate client and server network managers? or make interface more transparent?
    pub fn send(&mut self, message: NetworkEvent) -> Result<(), Error> {
        match self.remote() {
            Some(peer) => NetworkManager::send_to_peer(message, peer),
            None => Err(Error(-1)),
        }
    }

    pub fn send_to_peer(message: NetworkEvent, mut peer: Peer<()>) -> Result<(), Error> {
        match serialize(&message) {
            Some(bytes) => {
                let packet = Packet::new(&bytes, PacketMode::ReliableSequenced)?;
                peer.send_packet(packet, 0)
            }
            None => Err(Error(-2)),
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
                let remote_addr = Address::new(addr, SERVER_FRONTEND_PORT);
                self.host.connect(&remote_addr, 1, 0)?;
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

    fn service(&mut self, timeout_ms: u32) -> Result<Option<NetworkResponse>, Error> {
        match self.host.service(timeout_ms)? {
            Some(Event::Receive {
                ref packet,
                ref sender,
                ..
            }) => {
                match deserialize(packet.data()) {
                    Some(event) => Ok(Some(NetworkResponse {
                        event,
                        peer: sender.clone(),
                    })),
                    None => {
                        // TODO: log invalid packets
                        Ok(None)
                    }
                }
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
