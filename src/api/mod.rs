use serde::Deserialize;

pub mod state;
pub mod effects;
mod panel_layout;
mod rhythm;

#[derive(Deserialize, Debug)]
pub struct BoundedValue {
    pub value: u32,
    pub max: u32,
    pub min: u32
}

#[derive(Deserialize, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub o: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NanoLeafAPI {
    pub name: String,
    pub serial_no: String,
    pub manufacturer: String,
    pub firmware_version: String,
    pub model: String,
    pub state: state::State,
    pub effects: effects::Effects,
    pub panel_layout: panel_layout::PanelLayout,
    pub rhythm: rhythm::Rhythm
}
