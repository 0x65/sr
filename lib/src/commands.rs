use serde::{Deserialize, Serialize};

// todo remove debug
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCommand {
    HEARTBEAT,
}
