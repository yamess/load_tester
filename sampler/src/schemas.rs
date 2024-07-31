use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RequestPayload {
    pub x1: f64,
    pub x2: f64,
}

#[derive(Debug, Serialize)]
pub struct ResponsePayload {
    pub y: f64,
}