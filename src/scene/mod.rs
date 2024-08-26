use raylib::prelude::RaylibDrawHandle;

use crate::asset::AssetManager;

pub mod mario;

pub trait Scene {
    fn update(&mut self, _: &mut RaylibDrawHandle, _: f32);
}
