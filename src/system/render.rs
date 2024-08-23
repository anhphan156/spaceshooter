use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::entity::Entity;

pub fn render(entities: &Vec<Entity>, draw_thread: &mut RaylibDrawHandle) {
    for e in entities.iter() {
        let position = e.transform.position;
        draw_thread.draw_circle(position.x as i32, position.y as i32, 30.0, Color::BLACK);
        //draw_thread.draw_circle(600, 400, 200.0, Color::BLACK);
    }
}
