use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Default)]
pub struct AssetStore {
    assets: HashMap<u32, Vec<u8>>,
}

impl AssetStore {
    pub fn load_or_insert(&mut self, id: u32, file_path: &str) -> Result<&[u8], String> {
        if !self.assets.contains_key(&id) {
            let mut file = File::open(Path::new(file_path)).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;

            self.assets.insert(id, buffer);
        }
        Ok(self.assets.get(&id).unwrap())
    }

    pub fn get(&self, id: u32) -> Option<&[u8]> {
        self.assets.get(&id).map(|v| v.as_slice())
    }
}
