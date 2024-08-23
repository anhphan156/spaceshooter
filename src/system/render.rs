use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::entity::Entity;

pub fn render(entities: &Vec<Entity>, draw_thread: &mut RaylibDrawHandle) {
    for e in entities.iter() {
        if e.is_alive() {
            let position = e.transform.position;
            draw_thread.draw_circle(position.x as i32, position.y as i32, 5.0, Color::WHITE);
        }
    }
}
