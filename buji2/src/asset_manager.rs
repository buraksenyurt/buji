use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

// Texture'dan kurtaralım. u8 byte array'ler ile çalışsın
pub struct AssetManager<'a> {
    creator: &'a TextureCreator<WindowContext>,
    assets: HashMap<&'a str, Texture<'a>>,
}

impl<'a> AssetManager<'a> {
    pub fn new(creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            creator,
            assets: HashMap::new(),
        }
    }
    pub fn add(&mut self, id: &'a str, file_path: &str) -> Result<(), String> {
        if !self.assets.contains_key(id) {
            let texture = self.creator.load_texture(file_path)?;
            self.assets.insert(id, texture);
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Texture> {
        self.assets.get(id)
    }
}
