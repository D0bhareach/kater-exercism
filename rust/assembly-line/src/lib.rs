// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
const CARS_PER_HOUR: u16 = 221;
pub fn production_rate_per_hour(speed: u8) -> f64 {
let productivity = |speed|-> f64 {(CARS_PER_HOUR * speed as u16) as f64};
    match speed {
        0..=4 => productivity(speed),
        5..=8 => productivity(speed) * 0.9,
        9 | 10 => productivity(speed) *0.77,
        _ => 0.0
    }
    
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0 ) as u32
}
