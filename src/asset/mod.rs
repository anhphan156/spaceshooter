use core::panic;
use std::collections::HashMap;

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub struct AssetManager {
    pub textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> AssetManager {
        let mut textures: HashMap<String, Texture2D> = HashMap::new();

        let texture_names = ["ground", "mega_run", "mega_stand", "mega_jump", "coinspin"];
        for name in texture_names {
            let ground = rl.load_texture(
                thread,
                format!("/home/backspace/data/dev/spaceshooter/assets/{}.png", name).as_str(),
            );
            if let Ok(texture) = ground {
                let _ = textures.insert(name.to_string(), texture);
            } else {
                panic!("Failed to load texture")
            }
        }

        AssetManager { textures }
    }
}
