use crate::color::{color_to_u32, Color};
use std::sync::atomic::{AtomicU32, Ordering};

/// Frame buffer holding pixels
pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,

    // Shared framebuffer for the live preview. Each pixel holds a packed
    // 0x00RRGGBB value and is written by exactly one thread, so relaxed
    // atomics are enough to publish progress to the window loop.
    pixels: Vec<AtomicU32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            width,
            height,

            pixels: (0..width * height).map(|_| AtomicU32::new(0)).collect()
        }
    }

    pub fn write_pixel(&self, x: u32, y: u32, pixel: Color) {
        let color_u32 = color_to_u32(pixel);
        let idx = y as usize * self.width + x as usize;
        self.pixels[idx].store(color_u32, Ordering::Relaxed);
    }

    pub fn get_pixels(&self) -> impl Iterator<Item = u32> {
        self.pixels.iter().map(|p| p.load(Ordering::Relaxed))
    }
}
