use super::health_track::HealthTrack;
use serde::{Deserialize, Serialize};

/// HealthTracks Struct
#[derive(Serialize, Deserialize, Debug)]
pub struct HealthTracks {
    pub healthy: HealthTrack,
    pub injured: HealthTrack,
    pub incapacitated: HealthTrack,
}
