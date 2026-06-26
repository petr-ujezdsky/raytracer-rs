use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

/// Type alias for Vec3 used to represent an RGB color.
/// Components are expected to be in the range [0.0, 1.0].
pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 { linear_component.sqrt() } else { 0.0 }
}

/// Writes a single color to the given buffer in PPM format (three integers 0-255).
pub fn write_color<W: Write>(writer: &mut W, pixel_color: Color) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    const INTENSITY: Interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as u32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u32;

    writeln!(writer, "{} {} {}", rbyte, gbyte, bbyte).expect("Failed to write color");
}
