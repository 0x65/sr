use enet::{Address, BandwidthLimit, ChannelLimit, Enet, Error, InitializationError};

use crate::network::config::NetworkConfig;
use crate::network::constants::{FRONTEND_PORT, NUM_CHANNELS};
use crate::network::manager::NetworkManager;

pub mod config;
pub mod constants;
pub mod manager;
pub mod message;

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
                .map(|a| Address::new(a, FRONTEND_PORT))
                .as_ref(),
            NUM_CHANNELS,
            ChannelLimit::Maximum,
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )?;
        Ok(NetworkManager::new(config, host))
    }
}
