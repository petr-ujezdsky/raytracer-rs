use std::io::Write;

use crate::vec3::Vec3;

/// Type alias for Vec3 used to represent an RGB color.
/// Components are expected to be in the range [0.0, 1.0].
pub type Color = Vec3;

/// Writes a single color to the given buffer in PPM format (three integers 0-255).
pub fn write_color<W: Write>(writer: &mut W, pixel_color: Color) {
    // Translate the [0,1] component values to the byte range [0,255]
    let ir = (255.999 * pixel_color.x) as u32;
    let ig = (255.999 * pixel_color.y) as u32;
    let ib = (255.999 * pixel_color.z) as u32;

    writeln!(writer, "{} {} {}", ir, ig, ib).expect("Failed to write color");
}

