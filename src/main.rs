use std::fs::File;
use std::io::{BufWriter, Write};

mod vec3;

mod color;
use color::{Color, write_color};

fn main() {
    println!("Printing image");

    write_to_file(256, 256);
}

fn write_to_file(width: u32, height: u32) {
    // Create (or overwrite) the file and wrap it in a buffer for efficient incremental writing
    let file = File::create("output.ppm").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // First write the header
    writeln!(writer, "P3").expect("Failed to write header");
    writeln!(writer, "{} {}", width, height).expect("Failed to write header");
    writeln!(writer, "255").expect("Failed to write header");

    // Then write data incrementally in a for loop
    for j in 0..height {
        println!("Scanlines remaining {}", height - j);

        for i in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (height - 1) as f64;
            let b = 0f64;

            let pixel_color = Color { x: r, y: g, z: b };
            write_color(&mut writer, pixel_color);
        }
    }

    // Flush the buffer to make sure everything is actually written to disk
    writer.flush().expect("Failed to flush buffer");
    println!("Done 🎉");
}