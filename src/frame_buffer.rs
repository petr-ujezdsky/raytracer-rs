use crate::color::{color_to_u32, Color};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::atomic::{AtomicU32, Ordering};

/// Frame buffer holding pixels
pub struct FrameBuffer {
    width: usize,
    height: usize,

    tile_width: u32,
    tile_height: u32,

    tiles_count_vertical: u32,
    tiles_count_horizontal: u32,
    pub tiles_count_total: u32,

    // Shared framebuffer for the live preview. Each pixel holds a packed
    // 0x00RRGGBB value and is written by exactly one thread, so relaxed
    // atomics are enough to publish progress to the window loop.
    pixels: Vec<AtomicU32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        let tile_width = 50;
        let tile_height = 50;

        // let tile_width = self.image_width / 4;
        // let tile_height = self.image_height / 4;

        // let tile_width = self.image_width;
        // let tile_height = 1;

        // let tile_width = 1;
        // let tile_height = 1;

        let tiles_count_vertical = (height as f64 / tile_height as f64).ceil() as u32;
        let tiles_count_horizontal = (width as f64 / tile_width as f64).ceil() as u32;
        let tiles_count_total = tiles_count_vertical * tiles_count_horizontal;

        FrameBuffer {
            width,
            height,

            tile_width,
            tile_height,

            tiles_count_horizontal,
            tiles_count_vertical,
            tiles_count_total,

            pixels: (0..width * height).map(|_| AtomicU32::new(0)).collect()
        }
    }

    pub fn write_pixel(&self, x: u32, y: u32, pixel: Color) {
        let color_u32 = color_to_u32(pixel);
        let idx = y as usize * self.width + x as usize;
        self.pixels[idx].store(color_u32, Ordering::Relaxed);
    }

    pub fn get_tiles(&self) -> impl ParallelIterator<Item = Tile> + '_ {
        // Render the image in parallel
        (0..self.tiles_count_total)
            .into_par_iter()
            .map(|tile_index| {
                let i = tile_index % self.tiles_count_horizontal;
                let j = tile_index / self.tiles_count_vertical;

                let x = i * self.tile_width;
                let y = j * self.tile_height;

                let width = u32::min(self.tile_width, self.width as u32 - x);
                let height = u32::min(self.tile_height, self.height as u32 - y);

                Tile {
                    index: tile_index,
                    x,
                    y,
                    width,
                    height,
                }
            })
    }

    pub fn get_pixels(&self) -> impl Iterator<Item = u32> {
        self.pixels.iter().map(|p| p.load(Ordering::Relaxed))
    }
}

pub struct Tile {
    pub index: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
