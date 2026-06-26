use std::cmp::max;
use std::fs::File;
use std::io::{BufWriter, Write};

mod vec3;

mod color;
use color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod ray;

fn main() {
    println!("Printing image");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = max(1, (image_width as f64 / aspect_ratio) as i32);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    // Viewport widths less than one are ok since they are real valued.
    let viewport_width = viewport_height * ((image_width as f64) / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Create (or overwrite) the file and wrap it in a buffer for efficient incremental writing
    let file = File::create("output.ppm").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // First write the header
    writeln!(writer, "P3").expect("Failed to write header");
    writeln!(writer, "{} {}", image_width, image_height).expect("Failed to write header");
    writeln!(writer, "255").expect("Failed to write header");

    // Then write data incrementally in a for loop
    for j in 0..image_height {
        println!("Scanlines remaining {}", image_height - j);

        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray { origin: camera_center, direction: ray_direction };

            let pixel_color = ray_color(r);
            write_color(&mut writer, pixel_color);
        }
    }

    // Flush the buffer to make sure everything is actually written to disk
    writer.flush().expect("Failed to flush buffer");
    println!("Done");
    // write_to_file(256, 256);
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = Vec3::unit_vector(r.direction);
    let a = 0.5*(unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = center - r.origin;
    let a = Vec3::dot(r.direction, r.direction);
    let b = -2.0 * Vec3::dot(r.direction, oc);
    let c = Vec3::dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
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