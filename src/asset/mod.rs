use core::panic;
use std::collections::HashMap;

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub struct AssetManager {
    pub textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> AssetManager {
        let mut textures: HashMap<String, Texture2D> = HashMap::new();

        let ground = rl.load_texture(
            thread,
            "/home/backspace/data/dev/spaceshooter/assets/ground.png",
        );
        if let Ok(texture) = ground {
            let _ = textures.insert("ground".to_string(), texture);
        } else {
            panic!("Failed to load texture")
        }

        AssetManager { textures }
    }
}
