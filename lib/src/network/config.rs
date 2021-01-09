use std::net::Ipv4Addr;

pub struct NetworkConfig {
    pub local_addr: Option<Ipv4Addr>,
    pub remote_addr: Option<Ipv4Addr>,
}
