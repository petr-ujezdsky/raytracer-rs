use std::fs::File;
use std::io::{BufWriter, Write};

mod vec3;

mod color;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use color::{write_color, Color};

mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;

fn main() {
    println!("Printing image");

    // World
    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let camera = Camera::new(16.0 / 9.0, 400, 10);

    camera.render(&world);
    // write_to_file(256, 256);
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

            let pixel_color = Color::new(r, g, b);
            write_color(&mut writer, pixel_color);
        }
    }

    // Flush the buffer to make sure everything is actually written to disk
    writer.flush().expect("Failed to flush buffer");
    println!("Done ??");
}