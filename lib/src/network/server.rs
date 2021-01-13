use enet::{Error, Event, Host, Peer};

use crate::network::event::{deserialize, NetworkEvent};

#[derive(Debug)]
pub struct Request<'a> {
    pub event: NetworkEvent,
    pub peer: Peer<'a, ()>,
}

pub struct Server {
    host: Host<()>,
}

impl Server {
    pub fn new(host: Host<()>) -> Server {
        Server { host }
    }

    pub fn poll(&mut self) -> Result<Option<Request>, Error> {
        self.service(0)
    }

    fn service(&mut self, timeout_ms: u32) -> Result<Option<Request>, Error> {
        match self.host.service(timeout_ms)? {
            Some(Event::Receive {
                ref packet,
                ref sender,
                ..
            }) => {
                match deserialize(packet.data()) {
                    Some(event) => Ok(Some(Request {
                        event,
                        peer: sender.clone(),
                    })),
                    None => {
                        // TODO: log invalid packets
                        Ok(None)
                    }
                }
            }
            Some(Event::Connect(_)) => Ok(None),
            Some(Event::Disconnect(_, _)) => Ok(None),
            None => Ok(None),
        }
    }
}
