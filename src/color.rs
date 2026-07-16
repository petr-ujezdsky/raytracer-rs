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

/// Converts a color into a packed `0x00RRGGBB` value suitable for a `minifb` window buffer.
pub fn color_to_u32(pixel_color: Color) -> u32 {
    let r = linear_to_gamma(pixel_color.x);
    let g = linear_to_gamma(pixel_color.y);
    let b = linear_to_gamma(pixel_color.z);

    const INTENSITY: Interval = Interval::new(0.0, 0.999);
    let rbyte = (256.0 * INTENSITY.clamp(r)) as u32;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u32;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u32;

    (rbyte << 16) | (gbyte << 8) | bbyte
}

/// Converts a packed `0x00RRGGBB` value suitable for a `minifb` window buffer back into a `Color`.
pub fn u32_to_color(pixel_color: u32) -> Color {
    let r = ((pixel_color >> 16) & 0xFF) as f64 / 255.0;
    let g = ((pixel_color >> 8) & 0xFF) as f64 / 255.0;
    let b = (pixel_color & 0xFF) as f64 / 255.0;

    Color::new(r * r, g * g, b * b)
}
