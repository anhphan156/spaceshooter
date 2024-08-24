use raylib::prelude::RaylibDrawHandle;

pub mod mario;

pub trait Scene {
    fn update(&mut self, _: &mut RaylibDrawHandle, _: f32);
}
