use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::network::constants::{CLIENT_FRONTEND_PORT, SERVER_FRONTEND_PORT};

// TODO: document fields
pub struct NetworkConfig {
    pub local_addr: SocketAddr,
    pub remote_addr: Option<SocketAddr>,
}

impl NetworkConfig {
    pub fn client() -> NetworkConfig {
        NetworkConfig {
            local_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), CLIENT_FRONTEND_PORT),
            remote_addr: Some(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                SERVER_FRONTEND_PORT,
            )),
        }
    }

    pub fn server() -> NetworkConfig {
        NetworkConfig {
            local_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), SERVER_FRONTEND_PORT),
            remote_addr: None,
        }
    }
}
