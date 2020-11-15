use serde::Deserialize;
use crate::api::Position;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rhythm {
    pub rhythm_connected: bool,
    pub rhythm_active: Option<bool>,
    pub rhythm_id: Option<u32>,
    pub hardware_version: Option<String>,
    pub firmware_version: Option<String>,
    pub aux_available: Option<bool>,
    pub rhythm_mode: Option<u32>,
    pub rhythm_pos: Option<Position>
}
