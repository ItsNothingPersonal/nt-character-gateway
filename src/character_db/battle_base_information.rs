use super::health_tracks::HealthTracks;
use serde::{Deserialize, Serialize};

/// BattleBaseInformation Struct
#[derive(Serialize, Deserialize)]
pub struct BattleBaseInformation {
    pub initiative: u8,
    pub initiative_with_celerity: u8,
    pub health: HealthTracks,
}
