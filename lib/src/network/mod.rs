use enet::{Address, BandwidthLimit, ChannelLimit, Enet, Error, InitializationError};

use crate::network::config::NetworkConfig;
use crate::network::constants::SERVER_FRONTEND_PORT;
use crate::network::manager::NetworkManager;

pub mod config;
pub mod constants;
pub mod event;
pub mod manager;

pub struct Network {
    enet: Enet,
}

impl Network {
    pub fn new() -> Result<Network, InitializationError> {
        let enet = Enet::new()?;
        Ok(Network { enet })
    }

    pub fn create_manager(&self, config: NetworkConfig) -> Result<NetworkManager, Error> {
        let host = self.enet.create_host::<()>(
            config
                .local_addr
                .map(|a| Address::new(a, SERVER_FRONTEND_PORT))
                .as_ref(),
            config.max_peer_count,
            ChannelLimit::Maximum,
            config
                .bandwidth_incoming_limit_bytes_per_s
                .map_or(BandwidthLimit::Unlimited, BandwidthLimit::Limited),
            config
                .bandwidth_incoming_limit_bytes_per_s
                .map_or(BandwidthLimit::Unlimited, BandwidthLimit::Limited),
        )?;
        Ok(NetworkManager::new(config, host))
    }
}
