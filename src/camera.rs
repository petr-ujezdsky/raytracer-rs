use rayon::iter::ParallelIterator;
use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{Point3, Vec3};
use std::cmp::max;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use rayon::iter::IntoParallelIterator;
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

    /// Maximum number of ray bounces into scene
    pub max_depth: u32,

    /// Vertical view angle (field of view), in degrees
    pub vfov: u32,

    /// Point camera is looking from
    pub lookfrom: Point3,

    /// Point camera is looking at
    pub lookat: Point3,

    /// Camera-relative "up" direction
    pub vup: Vec3,

    /// Variation angle of rays through each pixel
    pub defocus_angle: f64,

    /// Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,

}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
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

    /// Maximum number of ray bounces into scene
    pub max_depth: u32,

    /// Vertical view angle (field of view), in degrees
    pub vfov: u32,

    /// Point camera is looking from
    pub lookfrom: Point3,

    /// Point camera is looking at
    pub lookat: Point3,

    /// Camera-relative "up" direction
    pub vup: Vec3,

    /// Variation angle of rays through each pixel
    pub defocus_angle: f64,

    /// Distance from camera lookfrom point to plane of perfect focus
    pub focus_dist: f64,

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

    /// Camera frame basis vectors - u
    u: Vec3,

    /// Camera frame basis vectors - v
    v: Vec3,

    /// Camera frame basis vectors - w
    w: Vec3,

    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3,

    /// Defocus disk vertical radius
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(config: CameraConfig) -> Camera {
        let CameraConfig { aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist } = config;

        // Calculate the image height, and ensure that it's at least 1.
        let image_height = max(1, (image_width as f64 / aspect_ratio) as u32);

        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = utils::degrees_to_radians(vfov as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        // Viewport widths less than one are ok since they are real valued.
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center
            - (focus_dist * w) - viewport_u/2 - viewport_v/2;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * f64::tan(utils::degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        // Construct camera
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
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

        let image_width_half = self.image_width / 2;

        // Track how many scanlines are already finished. Rows complete out of order
        // across threads, so the counter must be atomic.
        let scanlines_done = AtomicU32::new(0);
        let total = self.image_height;

        let pixels: Vec<Color> = (0..self.image_height)
            .into_par_iter()
            .flat_map(|j| {
                // Initialize random numbers generator *per thread*
                let mut rng = Random::from_os();

                let row: Vec<Color> = (0..self.image_width).map(|i| {
                    let mut pixel_color = Color::zero();

                    for _sample in 0..self.samples_per_pixel {
                        let r = self.get_ray(i, j, &mut rng);
                        let left_half = i < image_width_half;
                        // trace the ray and accumulate color
                        pixel_color += Self::ray_color(r, self.max_depth, world, &mut rng, left_half);
                    }

                    self.pixel_samples_scale * pixel_color
                }).collect();

                // Row finished: bump the counter and log remaining work.
                // fetch_add returns the value *before* incrementing, so add 1.
                let done = scanlines_done.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("Scanline #{:03} done, remaining: {}", j, total - done);

                row
            })
            .collect();

        // Then write data incrementally in a for loop
        for color in &pixels {
            write_color(&mut writer, *color);
        }

        // Flush the buffer to make sure everything is actually written to disk
        writer.flush().expect("Failed to flush buffer");
        println!("Done");
    }

    fn ray_color(r: Ray, depth: u32, world: &dyn Hittable, rng: &mut Random, left_half: bool) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        // Try to hit something in the world
        if let Some(rec) = world.hit(r, Interval::new(0.001, utils::INFINITY)) {
            // Use material
            let material = rec.mat_ptr.clone();
            if let Some(scatter_record) = material.scatter(r, &rec, rng) {
                return scatter_record.attenuation * Self::ray_color(scatter_record.scattered, depth - 1, world, rng, left_half);
            }

            return Color::zero();
        }

        // No hit -> background
        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5*(unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// Construct a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32, rng: &mut Random) -> Ray {
        let offset = self.sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample(rng) };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    fn sample_square(&self, rng: &mut Random) -> Vec3 {
        Vec3::new(rng.f64() - 0.5, rng.f64() - 0.5, 0.0)
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self, rng: &mut Random) -> Point3 {
        let p = Vec3::random_in_unit_disk(rng);
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
