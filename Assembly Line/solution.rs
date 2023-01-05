// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
pub fn production_rate_per_hour(speed: u8) -> f64 {
    const RATE: f64 = 221.00;
    const FULL: f64 = 1.00;
    const MEDIUM: f64 = 0.90;
    const LOW: f64 = 0.77;
    match speed {
        9..=10 => LOW * (speed as f64 * RATE),
        5..=8 => MEDIUM * (speed as f64 * RATE),
        _ => FULL * (speed as f64 * RATE),
    }
}
pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
