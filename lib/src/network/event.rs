use serde::{Deserialize, Serialize};

// TODO: document each event
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkEvent {
    Heartbeat,
    LoginRequest(String),
    LoginResponse(i64),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        for event in vec![
            NetworkEvent::Heartbeat,
            NetworkEvent::LoginRequest("test".to_string()),
            NetworkEvent::LoginResponse(123),
        ] {
            let val = deserialize(&serialize(&event).unwrap()).unwrap();
            assert_eq!(event, val);
        }
    }
}
