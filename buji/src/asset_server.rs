use image::*;
use std::io::Cursor;

/// A simple asset server struct for handling sprite sheet loading.
#[derive(Default)]
pub struct AssetServer;

impl AssetServer {
    /// Loads a sprite sheet from the specified source path and splits it into individual tiles.
    ///
    /// # Arguments
    ///
    /// * `source_path` - A string slice that holds the path of the image file to be loaded.
    /// * `tile_width` - The width of each tile in pixels.
    /// * `tile_height` - The height of each tile in pixels.
    ///
    /// # Returns
    ///
    /// A vector of tiles, where each tile is represented as a vector of bytes (PNG format).
    ///
    /// # Panics
    ///
    /// This function will panic if the image cannot be opened or if there is an error writing the tile data.
    pub fn load_atlas(source_path: &str, tile_width: u32, tile_height: u32) -> Vec<Vec<u8>> {
        let img = open(source_path).expect("Failed to open image");
        let (w, h) = img.dimensions();
        let mut tiles = Vec::new();

        for y in (0..h).step_by(tile_height as usize) {
            for x in (0..w).step_by(tile_width as usize) {
                let tile = img.view(x, y, tile_width, tile_height).to_image();
                let mut tile_bytes = Vec::new();
                let mut cursor = Cursor::new(&mut tile_bytes);
                tile.write_to(&mut cursor, ImageFormat::Png)
                    .expect("Write error");
                tiles.push(tile_bytes);
            }
        }

        tiles
    }
}
