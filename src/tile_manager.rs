use crate::color::{color_to_u32, Color};
use crate::frame_buffer::FrameBuffer;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Manager for tiling the image and tracking which tiles are currently being rendered
pub struct TileManager {
    frame_width: u32,
    frame_height: u32,

    tile_width: u32,
    tile_height: u32,

    tiles_count_horizontal: u32,
    pub tiles_count_total: u32,

    current_tiles_indices: Arc<Mutex<HashSet<u32>>>,
}

impl TileManager {
    pub fn new(tile_width: u32, tile_height: u32, frame_buffer: &FrameBuffer) -> TileManager {
        let frame_width = frame_buffer.width as u32;
        let frame_height = frame_buffer.height as u32;

        let tiles_count_horizontal = (frame_buffer.width as f64 / tile_width as f64).ceil() as u32;
        let tiles_count_vertical = (frame_buffer.height as f64 / tile_height as f64).ceil() as u32;
        let tiles_count_total = tiles_count_horizontal * tiles_count_vertical;

        TileManager {
            frame_width,
            frame_height,
            tile_width,
            tile_height,
            tiles_count_horizontal,
            tiles_count_total,
            current_tiles_indices: Arc::new(Mutex::new(HashSet::<u32>::new())),
        }
    }

    pub fn get_tiles_par_iter(&self) -> impl ParallelIterator<Item = Tile> {
        // Render the image in parallel
        (0..self.tiles_count_total)
            .into_par_iter()
            .map(|tile_index| self.create_tile(tile_index))
    }

    pub fn get_tiles_iter(&self) -> impl Iterator<Item = Tile> {
        (0..self.tiles_count_total)
            .map(|tile_index| self.create_tile(tile_index))
    }

    fn create_tile(&self, tile_index: u32) -> Tile {
        let i = tile_index % self.tiles_count_horizontal;
        let j = tile_index / self.tiles_count_horizontal;

        let x = i * self.tile_width;
        let y = j * self.tile_height;

        let width = u32::min(self.tile_width, self.frame_width - x);
        let height = u32::min(self.tile_height, self.frame_height - y);

        Tile::new(tile_index, x, y, width, height)
    }

    pub fn start_tile(&self, tile: &Tile) {
        self.current_tiles_indices.lock().unwrap().insert(tile.index);
    }

    pub fn finish_tile(&self, tile: &Tile) {
        self.current_tiles_indices.lock().unwrap().remove(&tile.index);
    }

    pub fn render_current_tiles_borders(&self, buffer: &mut Vec<u32>) {
        let indices: Vec<u32> = self.current_tiles_indices.lock().unwrap().iter().copied().collect();

        for tile_index in indices {
            let tile = self.create_tile(tile_index);

            // Draw marks - orange rectangle
            let mark_color = color_to_u32(Color::new(1.0, 0.65, 0.0));

            for (x, y) in tile.borders_iter() {
                let idx = (y * self.frame_width + x) as usize;
                buffer[idx] = mark_color;
            }
        }
    }
}

/// Tile in image that can be processed
pub struct Tile {
    pub index: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Tile {
    pub fn new(index: u32, x: u32, y: u32, width: u32, height: u32) -> Tile {
        Tile {
            index,
            x,
            y,
            width,
            height,
        }
    }

    pub fn pixels_iter(&self) -> impl Iterator<Item = (u32, u32)> {
        (0..self.height).flat_map(move |dy| {
            (0..self.width).map(move |dx| (self.x + dx, self.y + dy))
        })
    }

    pub fn borders_iter(&self) -> impl Iterator<Item = (u32, u32)> {
        let (x, y, w, h) = (self.x, self.y, self.width, self.height);

        // top and bottom edges (full width)
        let top = (0..w).map(move |dx| (x + dx, y));
        let bottom = (0..w).map(move |dx| (x + dx, y + h - 1));

        // left and right edges without corners (already covered by the top/bottom edges)
        let left = (1..h - 1).map(move |dy| (x, y + dy));
        let right = (1..h - 1).map(move |dy| (x + w - 1, y + dy));

        top.chain(right).chain(bottom).chain(left)
    }
}
