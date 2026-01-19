use std::f64;

pub fn degree_to_radians(degress: f64) -> f64 {
    return degress * f64::consts::PI / 180.0;
}
