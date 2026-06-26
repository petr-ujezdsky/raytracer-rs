use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{Point3, Vec3};
use std::cmp::max;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::random::Random;

/// Config struct for [`Camera`](Camera) that enables usage of default parameters
#[derive(Debug)]
pub struct CameraConfig {
    /// Ratio of image width over height
    pub aspect_ratio: f64,

    /// Rendered image width in pixel count
    pub image_width: u32,

    /// Count of random samples for each pixel
    pub samples_per_pixel: u32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    /// Ratio of image width over height
    pub aspect_ratio: f64,

    /// Rendered image width in pixel count
    pub image_width: u32,

    /// Count of random samples for each pixel
    pub samples_per_pixel: u32,

    /// Rendered image height
    image_height: u32,

    /// Camera center
    center: Point3,

    /// Location of pixel 0, 0
    pixel00_loc: Point3,

    /// Offset to pixel to the right
    pixel_delta_u: Vec3,

    /// Offset to pixel below
    pixel_delta_v: Vec3,

    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Camera {
        let CameraConfig { aspect_ratio, image_width, samples_per_pixel } = config;

        // Calculate the image height, and ensure that it's at least 1.
        let image_height = max(1, (image_width as f64 / aspect_ratio) as u32);

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        // Viewport widths less than one are ok since they are real valued.
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
            - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2 - viewport_v/2;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Construct camera
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        // Create (or overwrite) the file and wrap it in a buffer for efficient incremental writing
        let file = File::create("output.ppm").expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        // First write the header
        writeln!(writer, "P3").expect("Failed to write header");
        writeln!(writer, "{} {}", self.image_width, self.image_height).expect("Failed to write header");
        writeln!(writer, "255").expect("Failed to write header");

        // Initialize random numbers generator
        let mut rng = Random::from_os();

        // Then write data incrementally in a for loop
        for j in 0..self.image_height {
            println!("Scanlines remaining {}", self.image_height - j);

            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();

                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, &mut rng);
                    pixel_color += Camera::ray_color(r, world);
                }

                write_color(&mut writer, self.pixel_samples_scale * pixel_color);
            }
        }

        // Flush the buffer to make sure everything is actually written to disk
        writer.flush().expect("Failed to flush buffer");
        println!("Done");
    }

    fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, utils::INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        // no hit -> background
        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5*(unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled
    /// point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32, rng: &mut Random) -> Ray {
        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square(&self, rng: &mut Random) -> Vec3 {
        Vec3::new(rng.f64() - 0.5, rng.f64() - 0.5, 0.0)
    }
}
