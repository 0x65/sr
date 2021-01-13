use std::net::Ipv4Addr;

use enet::Address;

use crate::network::constants::{SERVER_DEFAULT_PEER_LIMIT, SERVER_FRONTEND_PORT};

// TODO: document fields

pub struct ClientConfig {
    pub remote_addr: Address,
    pub bandwidth_incoming_limit_bytes_per_s: Option<u32>,
    pub bandwidth_outgoing_limit_bytes_per_s: Option<u32>,
}

impl ClientConfig {
    pub fn default() -> ClientConfig {
        ClientConfig {
            remote_addr: Address::new(Ipv4Addr::LOCALHOST, SERVER_FRONTEND_PORT),
            bandwidth_incoming_limit_bytes_per_s: None,
            bandwidth_outgoing_limit_bytes_per_s: None,
        }
    }
}

pub struct ServerConfig {
    pub local_addr: Address,
    pub max_num_peers: usize,
    pub bandwidth_incoming_limit_bytes_per_s: Option<u32>,
    pub bandwidth_outgoing_limit_bytes_per_s: Option<u32>,
}

impl ServerConfig {
    pub fn default() -> ServerConfig {
        ServerConfig {
            local_addr: Address::new(Ipv4Addr::UNSPECIFIED, SERVER_FRONTEND_PORT),
            max_num_peers: SERVER_DEFAULT_PEER_LIMIT,
            bandwidth_incoming_limit_bytes_per_s: None,
            bandwidth_outgoing_limit_bytes_per_s: None,
        }
    }
}
