use serde::Serialize;

#[derive(Serialize)]
pub struct NanoLeafColor {
    hue: u32,
    saturation: u32,
    brightness: u32
}

impl NanoLeafColor {
    pub fn new(hue: u32, saturation: u32, brightness: u32) -> NanoLeafColor {
        NanoLeafColor { hue, saturation, brightness }
    }
}
