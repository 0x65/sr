use serde::{Deserialize, Serialize};

// todo remove debug
#[derive(Serialize, Deserialize)]
pub enum ClientCommand {
    HEARTBEAT,
}
