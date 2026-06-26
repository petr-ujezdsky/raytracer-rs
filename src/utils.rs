pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
// const PI: f64 = 3.1415;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
