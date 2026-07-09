use crate::color::Color;
use crate::vec3::Vec3;
use std::sync::Arc;
use image::{GenericImageView, RgbImage};
use crate::perlin::Perlin;
use crate::random::Random;

/// Surface texture.
pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self { albedo: Color::new(r, g, b) }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd,
            even,
        }
    }

    pub fn from_colors(scale: f64, odd: Color, even: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        let x_integer = (self.inv_scale * p.x).floor() as i32;
        let y_integer = (self.inv_scale * p.y).floor() as i32;
        let z_integer = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even { self.even.value(u, v, p) } else { self.odd.value(u, v, p) }
    }
}

pub struct ImageTexture {
    buffer: Option<RgbImage>,   // ImageBuffer<Rgb<u8>, Vec<u8>>
    width: f64,
    height: f64,
}

impl ImageTexture {
    pub fn new(path: &String) -> Self {
        match image::open(path) {
            Ok(image) => {
                let (width, height) = image.dimensions();

                Self {
                    buffer: Some(image.into_rgb8()),
                    width: width as f64,
                    height: height as f64,
                }
            },

            Err(e) => {
                eprintln!("Failed to load image texture from {}: {}", path, e);
                // Return a default solid color texture (cyan) in case of error
                Self {
                    buffer: None,
                    width: 0.0,
                    height: 0.0,
                }
            }
        }
    }

    // fn pixel_data(&self, x: u32, y: u32) -> Color {
    //     match self.image {
    //         Ok(ref image) => {
    //             let pixel = image.get_pixel(x, y);
    //             Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
    //         }
    //         _ => Color::new(0.0, 1.0, 1.0),
    //     }
    // }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        match self.buffer {
            Some(ref buffer) => {
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = u.clamp(0.0, 1.0);
                // Flip V to image coordinates
                let v = 1.0 - v.clamp(0.0, 1.0);

                let i = (u * self.width).floor() as u32;
                let j = (v * self.height).floor() as u32;
                let pixel = buffer.get_pixel(i, j);

                let color_scale = 1.0 / 255.0;
                Color::new(color_scale * pixel[0] as f64, color_scale * pixel[1] as f64, color_scale * pixel[2] as f64)
            }
            // If we have no texture data, then return solid cyan as a debugging aid.
            _ => Color::new(0.0, 1.0, 1.0),
        }
    }
}

pub struct NoiseTexture {
    scale : f64,
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(scale: f64, rng: &mut Random) -> Self {
        Self {
            scale,
            noise: Perlin::new(rng),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
        // map [-1; 1] to [0; 1]
        // Color::new(1.0,1.0,1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
        Color::new(0.5, 0.5, 0.5) * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(&p, 7)))

    }
}
