use crate::color::{color_to_u32, u32_to_color, write_color, Color};
use crate::frame_buffer::FrameBuffer;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::random::Random;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{Point3, Vec3};
use indicatif::{ProgressBar, ProgressStyle};
use minifb::{Key, Window, WindowOptions};
use rayon::iter::ParallelIterator;
use std::cmp::max;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::Instant;

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

    /// Scene background color
    pub background: Color,

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

    /// Optional seed for rng
    pub rng_seed: Option<u64>,

}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::zero(),
            vfov: 90,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            rng_seed: None,
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

    /// Scene background color
    pub background: Color,

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

    /// Optional seed for rng
    pub rng_seed: Option<u64>,

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
        let CameraConfig { aspect_ratio, image_width, samples_per_pixel, max_depth, background, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist, rng_seed } = config;

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
            background,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            rng_seed,
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
        let width = self.image_width as usize;
        let height = self.image_height as usize;

        // Shared framebuffer for the live preview. Each pixel holds a packed
        // 0x00RRGGBB value and is written by exactly one thread, so relaxed
        // atomics are enough to publish progress to the window loop.
        let mut framebuffer = FrameBuffer::new(width, height);

        let render_done = AtomicBool::new(false);
        // Set when the preview window is closed so the render can bail out early.
        let cancelled = AtomicBool::new(false);

        std::thread::scope(|scope| {
            // Do the actual rendering (and file writing) on a background thread
            // so the window event loop on the main thread stays responsive.
            scope.spawn(|| {
                // Start measuring the total render time.
                let start = Instant::now();

                // Init progress bar (total pixels count)
                let progress_bar = ProgressBar::new((self.image_height * self.image_width) as u64);
                progress_bar.set_style(
                    ProgressStyle::with_template(
                        "{spinner:.green} [{bar:30.cyan/blue}] {pos}/{len} ({eta}) {msg}",
                    )
                        .unwrap()
                        .progress_chars("#>-"),
                );

                // Track how many tiles are already finished. tiles complete out of order
                // across threads, so the counter must be atomic.
                let tiles_count_done = AtomicU32::new(0);

                // Render the image in parallel
                framebuffer.get_tiles()
                    .for_each(|tile| {
                        // Initialize random numbers generator *per thread*
                        let mut rng = match self.rng_seed {
                            Some(s) => Random::new(tile.index as u64 * s),
                            None => Random::from_os(),
                        };

                        let mut tile_buffer = vec![0u32; (tile.width * tile.height) as usize];

                        // Draw marks - orange rectangle
                        let mark_color = Color::new(1.0, 0.65, 0.0);
                        let (x0, y0) = (tile.x, tile.y);
                        let (x1, y1) = (tile.x + tile.width - 1, tile.y + tile.height - 1);

                        // horizontal edges
                        for x in x0..=x1 {
                            framebuffer.write_pixel(x, y0, mark_color);
                            framebuffer.write_pixel(x, y1, mark_color);
                        }
                        // vertical edges
                        for y in y0..=y1 {
                            framebuffer.write_pixel(x0, y, mark_color);
                            framebuffer.write_pixel(x1, y, mark_color);
                        }

                        for t_j in 0..tile.height {
                            for t_i in 0..tile.width {
                                // Bail out fast on every remaining pixel once cancelled.
                                if cancelled.load(Ordering::Relaxed) {
                                    return;
                                }

                                // Global coordinates
                                let i = tile.x + t_i;
                                let j = tile.y + t_j;

                                let mut pixel_color = Color::zero();

                                for _sample in 0..self.samples_per_pixel {
                                    let r = self.get_ray(i, j, &mut rng);
                                    // trace the ray and accumulate color
                                    pixel_color += self.ray_color(r, self.max_depth, world, &mut rng);
                                }

                                let color = self.pixel_samples_scale * pixel_color;

                                // Publish the finished pixel to the live preview buffer (do not overwrite marks)
                                if (i != x0 && i != x1 && j != y0 && j != y1) {
                                    framebuffer.write_pixel(i, j, color);
                                }

                                // Store in local tile buffer
                                let idx_tile = (t_j * tile.width + t_i) as usize;
                                tile_buffer[idx_tile] = color_to_u32(color);

                                progress_bar.inc(1);
                            }
                        }

                        // Re-copy tile buffer to "paint over" the marks
                        for (i, color) in tile_buffer.iter().enumerate() {
                            let x = tile.x + i as u32 % tile.width;
                            let y = tile.y + i as u32 / tile.width;

                            framebuffer.write_pixel(x, y, u32_to_color(*color));
                        }

                        // Tile finished: bump the counter and log remaining work.
                        // fetch_add returns the value *before* incrementing, so add 1.
                        let done = tiles_count_done.fetch_add(1, Ordering::Relaxed) + 1;
                        progress_bar.println(&format!("Tile #{:03} done, remaining: {}", tile.index+1, framebuffer.tiles_count_total - done));
                    });

                // If the preview window was closed mid-render, drop the partial
                // result instead of writing an incomplete image to disk.
                if cancelled.load(Ordering::Relaxed) {
                    progress_bar.abandon_with_message(format!("Render cancelled after {:.2?}", start.elapsed()));
                    return;
                }

                // Create (or overwrite) the file and wrap it in a buffer for efficient incremental writing
                let file = File::create("output.ppm").expect("Failed to create file");
                let mut writer = BufWriter::new(file);

                // First write the header
                writeln!(writer, "P3").expect("Failed to write header");
                writeln!(writer, "{} {}", self.image_width, self.image_height).expect("Failed to write header");
                writeln!(writer, "255").expect("Failed to write header");

                // Then write data incrementally in a for loop
                for color_u32 in framebuffer.get_pixels() {
                    let color = u32_to_color(color_u32);
                    write_color(&mut writer, color);
                }

                // Flush the buffer to make sure everything is actually written to disk
                writer.flush().expect("Failed to flush buffer");

                // log elapsed time - "?" (debug) is in dynamic units (1.23s, 350.00ms, 12.50µs)
                progress_bar.finish_with_message(format!("Done in {:.2?}", start.elapsed()));

                render_done.store(true, Ordering::Relaxed);
            });

            // Live preview window on the main thread (required on macOS).
            let mut window = Window::new(
                "raytracer-rs — rendering (Esc to close)",
                width,
                height,
                WindowOptions::default(),
            ).expect("Failed to open preview window");

            let mut buffer = vec![0u32; width * height];
            let mut finished_shown = false;
            let mut last_snapshot = Instant::now();
            // Re-copy the image at ~30 FPS; the window itself is refreshed (and
            // events pumped) every iteration via update_with_buffer.
            let snapshot_interval = std::time::Duration::from_millis(33);

            while window.is_open() && !window.is_key_down(Key::Escape) {
                let done = render_done.load(Ordering::Relaxed);

                // Snapshot the shared framebuffer into the window buffer while the
                // render is still running (throttled), and once more when it finishes.
                if !finished_shown && (last_snapshot.elapsed() >= snapshot_interval || done) {
                    for (dst, src) in buffer.iter_mut().zip(framebuffer.get_pixels()) {
                        *dst = src;
                    }
                    last_snapshot = Instant::now();

                    if done {
                        window.set_title("raytracer-rs — done (Esc to close)");
                        finished_shown = true;
                    }
                }

                // Always drive the window through update_with_buffer only (mixing it
                // with update() on the same window is not supported by minifb).
                window.update_with_buffer(&buffer, width, height).expect("Failed to update window");

                // Yield briefly instead of busy-spinning (~250 Hz event pump).
                std::thread::sleep(std::time::Duration::from_millis(4));
            }

            // Window closed: signal the render thread to stop. The scope then
            // joins the background thread before returning.
            cancelled.store(true, Ordering::Relaxed);
        });
    }

    fn ray_color(&self, r: Ray, depth: u32, world: &dyn Hittable, rng: &mut Random) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::zero();
        }

        // Try to hit something in the world
        if let Some(rec) = world.hit(r, Interval::new(0.001, utils::INFINITY), rng) {
            // Use material
            let material = rec.mat_ptr;
            let color_from_emission = material.emitted(rec.u, rec.v, rec.p);

            if let Some(scatter_record) = material.scatter(r, &rec, rng) {
                let color_from_scatter = scatter_record.attenuation * self.ray_color(scatter_record.scattered, depth - 1, world, rng);

                // Scattered -> combine both colors
                return color_from_emission + color_from_scatter;
            }

            // No scatter -> just emission
            return color_from_emission;
        }

        // No hit -> background
        self.background
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
        let ray_time = rng.f64();

        Ray::new(ray_origin, ray_direction, ray_time)
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
