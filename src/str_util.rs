extern crate math;

use math::round;

pub fn to_fixed(value: f64, scale: i8) -> String {
    let rounded = round::half_away_from_zero(value, scale);
    return format!("{}", rounded);
}