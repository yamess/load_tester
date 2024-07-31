use actix::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Message)]
#[rtype(result = "Vec<u8>")]
pub struct LoadTestPayload {
    pub host: String,
    pub users: usize,
    pub runtime: usize,
}

