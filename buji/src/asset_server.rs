use crate::{Log, LogLevel};
use image::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::rc::Rc;

/// A simple asset server struct for handling sprite sheet loading.
pub struct AssetServer<W: Write> {
    /// Vector based tile map of sprite sheet
    pub tile_map: HashMap<usize, Vec<u8>>,
    pub logger: Option<Rc<RefCell<Log<W>>>>,
}

impl<W: Write> AssetServer<W> {
    /// Creates a new AssetServer instance.
    ///
    /// # Arguments
    ///
    /// * `logger` - An optional logger to log events during tile sheet loading.
    ///
    pub fn new(logger: Option<Rc<RefCell<Log<W>>>>) -> Self {
        Self {
            tile_map: HashMap::new(),
            logger,
        }
    }

    /// Loads a sprite sheet from the specified source path and splits it into individual tiles.
    ///
    /// # Arguments
    ///
    /// * `source_path` - A string slice that holds the path of the image file to be loaded.
    /// * `tile_width` - The width of each tile in pixels.
    /// * `tile_height` - The height of each tile in pixels.
    /// * `columns` - The number of columns to load.
    /// * `rows` - The number of rows to load.
    ///
    /// # Panics
    ///
    /// This function will panic if the image cannot be opened or if there is an error writing the tile data.
    pub fn init(
        &mut self,
        source_path: &str,
        tile_width: u32,
        tile_height: u32,
        columns: u32,
        rows: u32,
    ) {
        if let Some(logger) = &self.logger {
            let mut logger_ref = logger.borrow_mut();
            logger_ref.write(LogLevel::Info, "Initializing AssetServer");
            logger_ref.write(LogLevel::Info, &format!("source_path: {}", source_path));
        }

        let img = open(source_path).expect("Failed to open image");
        let (w, h) = img.dimensions();

        let max_width = columns * tile_width;
        let max_height = rows * tile_height;

        let final_width = max_width.min(w);
        let final_height = max_height.min(h);

        let mut index = 0;

        for y in (0..final_height).step_by(tile_height as usize) {
            for x in (0..final_width).step_by(tile_width as usize) {
                let tile = img.view(x, y, tile_width, tile_height).to_image();
                let mut tile_bytes = Vec::new();
                let mut cursor = Cursor::new(&mut tile_bytes);
                tile.write_to(&mut cursor, image::ImageFormat::Png)
                    .expect("Write error");
                self.tile_map.insert(index, tile_bytes);
                index += 1;
            }
        }
    }

    /// Loads a sprite sheet from the hash store
    ///
    /// # Arguments
    ///
    /// * `index` - Index number of sprite
    ///
    pub fn get_sprite(&self, index: usize) -> Option<&Vec<u8>> {
        self.tile_map.get(&index)
    }
}
