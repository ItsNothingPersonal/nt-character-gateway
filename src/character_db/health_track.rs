use serde::{Deserialize, Serialize};

/// HealthTrack Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct HealthTrack {
    pub base_value: u8,
    pub with_boni: u8,
    pub remaining: u8,
}
