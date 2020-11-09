use std::vec::Vec;
use serde::Deserialize;
use crate::api::{BoundedValue, PanelPos};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub panel_id: u32,
    pub shape_type: u32,
    
    #[serde(flatten)]
    pub pos: PanelPos,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub num_panels: u32,
    pub side_length: u32,
    pub position_data: Vec<Position>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PanelLayout {
    pub layout: Layout,
    pub global_orientation: BoundedValue,
}
