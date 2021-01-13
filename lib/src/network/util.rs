use enet::{Error, Packet, PacketMode, Peer};

use crate::network::event::{serialize, NetworkEvent};

pub fn send_to_peer(mut peer: Peer<()>, message: NetworkEvent) -> Result<(), Error> {
    match serialize(&message) {
        Some(bytes) => {
            let packet = Packet::new(&bytes, PacketMode::ReliableSequenced)?;
            peer.send_packet(packet, 0)
        }
        None => Err(Error(-2)),
    }
}
