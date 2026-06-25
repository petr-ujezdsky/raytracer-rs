use std::fs::File;
use std::io::{BufWriter, Write};

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
        for i in 0..width {
            let r = i as f32 / (width - 1) as f32;
            let g = j as f32 / (height - 1) as f32;
            let b = 0f32;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            writeln!(writer, "{} {} {}", ir, ig, ib).expect("Failed to write data");
        }
    }

    // Flush the buffer to make sure everything is actually written to disk
    writer.flush().expect("Failed to flush buffer");
}