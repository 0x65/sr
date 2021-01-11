use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkEvent {
    LoginRequest(String)
}

pub fn serialize(event: &NetworkEvent) -> Option<Vec<u8>> {
    match bincode::serialize(event) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn deserialize(bytes: &[u8]) -> Option<NetworkEvent> {
    match bincode::deserialize(bytes) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}
