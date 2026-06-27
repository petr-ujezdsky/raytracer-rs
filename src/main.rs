use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

mod vec3;

mod color;
use crate::camera::{Camera, CameraConfig};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use color::{write_color, Color};
use crate::material::{Lambertian, Metal};

mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod random;
mod material;

fn main() {
    println!("Printing image");

    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    // World
    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new( 0.0,    0.0, -1.2),   0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left));
    world.add(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        ..Default::default()
    });

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