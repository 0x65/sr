use enet::{BandwidthLimit, ChannelLimit, Enet, Error, InitializationError};

use crate::network::client::Client;
use crate::network::config::{ClientConfig, ServerConfig};
use crate::network::server::Server;

pub mod client;
pub mod config;
pub mod constants;
pub mod event;
pub mod server;
pub mod util;

pub struct Network {
    enet: Enet,
}

impl Network {
    pub fn new() -> Result<Network, InitializationError> {
        let enet = Enet::new()?;
        Ok(Network { enet })
    }

    pub fn create_client(&self, config: ClientConfig) -> Result<Client, Error> {
        let host = self.enet.create_host::<()>(
            None, // local addr
            1,    // max num peers
            ChannelLimit::Maximum,
            convert_limit(config.bandwidth_incoming_limit_bytes_per_s),
            convert_limit(config.bandwidth_outgoing_limit_bytes_per_s),
        )?;
        Ok(Client::new(config, host))
    }

    pub fn create_server(&self, config: ServerConfig) -> Result<Server, Error> {
        let host = self.enet.create_host::<()>(
            Some(&config.local_addr),
            config.max_num_peers,
            ChannelLimit::Maximum,
            convert_limit(config.bandwidth_incoming_limit_bytes_per_s),
            convert_limit(config.bandwidth_outgoing_limit_bytes_per_s),
        )?;
        Ok(Server::new(host))
    }
}

#[inline]
fn convert_limit(limit: Option<u32>) -> BandwidthLimit {
    limit.map_or(BandwidthLimit::Unlimited, BandwidthLimit::Limited)
}
