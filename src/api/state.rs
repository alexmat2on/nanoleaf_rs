use serde::{Serialize, Deserialize};
use crate::api::BoundedValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct On { pub value: bool }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub on: On,
    pub brightness: BoundedValue,
    pub hue: BoundedValue,
    pub sat: BoundedValue,
    pub ct: BoundedValue,
    pub color_mode: String
}
