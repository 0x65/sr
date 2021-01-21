use std::net::SocketAddr;
use std::thread;

use crossbeam_channel::{Receiver, Sender, TryRecvError};
use laminar::{ErrorKind, Packet, Result, Socket, SocketEvent};

use crate::message::{deserialize, serialize, Message};
use crate::network::config::NetworkConfig;

pub mod config;
pub mod constants;

pub enum NetworkEvent {
    Message(Message, SocketAddr),
    Connect(SocketAddr),
    Timeout(SocketAddr),
    Disconnect(SocketAddr),
}

// TODO: NETWORK ERROR
pub struct Network {
    config: NetworkConfig,
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    thread_handle: thread::JoinHandle<()>,
}

impl Network {
    pub fn new(config: NetworkConfig) -> Result<Network> {
        let mut socket = Socket::bind(config.local_addr)?;
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        let thread_handle = thread::spawn(move || socket.start_polling());
        Ok(Network {
            config,
            sender,
            receiver,
            thread_handle,
        })
    }

    pub fn recv(&self) -> Result<Option<NetworkEvent>> {
        match self.receiver.try_recv() {
            Ok(socket_event) => match socket_event {
                SocketEvent::Packet(p) => Network::read_packet(p).map(Option::Some),
                SocketEvent::Connect(a) => Ok(Some(NetworkEvent::Connect(a))),
                SocketEvent::Timeout(a) => Ok(Some(NetworkEvent::Timeout(a))),
                SocketEvent::Disconnect(a) => Ok(Some(NetworkEvent::Disconnect(a))),
            },
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(ErrorKind::ReceivedDataToShort), // TODO: FIX
        }
    }

    pub fn send(&mut self, msg: Message) -> Result<()> {
        match self.config.remote_addr {
            Some(peer) => self.send_to_peer(msg, peer),
            None => Err(ErrorKind::ReceivedDataToShort), // TODO: fix
        }
    }

    pub fn send_to_peer(&mut self, msg: Message, peer: SocketAddr) -> Result<()> {
        let serialized = serialize(&msg).unwrap(); // TODO fix
        let packet = Packet::reliable_unordered(peer, serialized);
        self.sender.send(packet).unwrap(); // TODO fix
        Ok(())
    }

    fn read_packet(packet: Packet) -> Result<NetworkEvent> {
        let deserialized = deserialize(packet.payload()).unwrap(); // TODO: fix
        Ok(NetworkEvent::Message(deserialized, packet.addr()))
    }
}
