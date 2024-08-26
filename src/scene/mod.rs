use raylib::{RaylibHandle, RaylibThread};

pub mod mario;

pub trait Scene {
    fn update(&mut self, _: &mut RaylibHandle, _: &mut RaylibThread);
}
