use std::net::Ipv4Addr;

use crate::network::constants::SERVER_DEFAULT_PEER_LIMIT;

// TODO: document fields
pub struct NetworkConfig {
    pub local_addr: Option<Ipv4Addr>,
    pub remote_addr: Option<Ipv4Addr>,
    pub max_peer_count: usize,
    pub bandwidth_incoming_limit_bytes_per_s: Option<u32>,
    pub bandwidth_outgoing_limit_bytes_per_s: Option<u32>,
}

impl NetworkConfig {
    pub fn default(local_addr: Option<Ipv4Addr>, remote_addr: Option<Ipv4Addr>) -> NetworkConfig {
        NetworkConfig {
            local_addr,
            remote_addr,
            max_peer_count: SERVER_DEFAULT_PEER_LIMIT,
            bandwidth_incoming_limit_bytes_per_s: None,
            bandwidth_outgoing_limit_bytes_per_s: None,
        }
    }
}
