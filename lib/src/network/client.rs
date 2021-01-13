use enet::{Error, Event, Host, Peer};

use crate::network::config::ClientConfig;
use crate::network::event::{deserialize, NetworkEvent};
use crate::network::util::send_to_peer;

#[derive(Debug)]
pub struct Response {
    pub event: NetworkEvent,
}

enum ClientState {
    Unconnected,
    Connecting,
    Ready,
}

pub struct Client {
    config: ClientConfig,
    state: ClientState,
    host: Host<()>,
}

// TODO: handle unexpected events (e.g. receive on connecting)
// TODO: close connection (make sure can read disconnect msg on server on 'q')
impl Client {
    pub fn new(config: ClientConfig, host: Host<()>) -> Client {
        Client {
            config,
            state: ClientState::Unconnected,
            host,
        }
    }

    pub fn poll(&mut self) -> Result<Option<Response>, Error> {
        match self.state {
            ClientState::Unconnected => {
                self.connect()?;
                Ok(None)
            }
            ClientState::Connecting | ClientState::Ready => self.service(0),
        }
    }

    pub fn send(&mut self, message: NetworkEvent) -> Result<(), Error> {
        match self.remote() {
            Some(peer) => send_to_peer(peer, message),
            None => Err(Error(-1)),
        }
    }

    fn connect(&mut self) -> Result<(), Error> {
        self.state = ClientState::Connecting;
        self.host.connect(&self.config.remote_addr, 1, 0)?;
        Ok(())
    }

    fn remote(&mut self) -> Option<Peer<()>> {
        match self.state {
            // based on max_num_peers = 1
            ClientState::Ready => self.host.peers().next(),
            _ => None,
        }
    }

    fn service(&mut self, timeout_ms: u32) -> Result<Option<Response>, Error> {
        match self.host.service(timeout_ms)? {
            Some(Event::Receive { ref packet, .. }) => {
                match deserialize(packet.data()) {
                    Some(event) => Ok(Some(Response { event })),
                    None => {
                        // TODO: log invalid packets
                        Ok(None)
                    }
                }
            }
            Some(Event::Connect(_)) => {
                self.state = ClientState::Ready;
                Ok(None)
            }
            Some(Event::Disconnect(_, _)) => {
                self.state = ClientState::Unconnected;
                Ok(None)
            }
            None => Ok(None),
        }
    }
}
