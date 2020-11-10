use crate::color::NanoLeafColor;
use std::vec::Vec;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Effects {
    pub select: String,
    pub effects_list: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectDetails<'a> {
    pub command: &'a str,
    pub version: &'a str,
    pub anim_type: &'a str,
    pub anim_name: &'a str,
    pub color_type: &'a str,
    pub plugin_uuid: &'a str,
    pub plugin_type: &'a str,
    pub plugin_options: Vec<PluginOption::<'a>>,
    pub palette: Vec<NanoLeafColor>,
    pub has_overlay: bool
}

#[derive(Serialize)]
#[serde(tag = "name", content = "value", rename_all = "camelCase")]
pub enum PluginOption<'a> {
    TransTime(u32),
    DelayTime(u32),
    LinDirection(&'a str),
    Loop(bool)
}
