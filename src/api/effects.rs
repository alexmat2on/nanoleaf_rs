use std::vec::Vec;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Effects {
    pub select: String,
    pub effects_list: Vec<String>,
}
