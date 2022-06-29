// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let min_rate = 221;
    let mut rate: f64 = 1.0;
    // 5-8 90% success rate
    // 9-10 77% success rate
    if (speed >= 5 && speed <= 8) {
      rate = 0.9;
    } else if (speed > 8 && speed <= 10) {
      rate = 0.77;
    }
    let car_per_hour: f64  = min_rate as f64 * rate * speed as f64;
    return car_per_hour;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let mut rate: u32 = production_rate_per_hour(speed) as u32 / 60;
    return rate ;
}
