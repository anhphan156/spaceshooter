use glam::Vec2;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    component::ctransform::CTransform,
    entity::{
        entity_manager::{self, EntityManager},
        Entity,
    },
    util::constant::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

use super::Scene;

pub struct MarioScene {
    pub entity_manager: EntityManager,
}

impl MarioScene {
    pub fn new() -> Self {
        let entity_manager = EntityManager::new();

        MarioScene { entity_manager }
    }
}

impl Scene for MarioScene {
    fn update(&mut self) {}
}

fn check_out_of_bound(entities: &mut Vec<Entity>) {
    for e in entities.iter_mut() {
        if !e.is_alive() {
            continue;
        }
        if e.transform.position.x < 0.0
            || e.transform.position.y < 0.0
            || e.transform.position.x > WINDOW_WIDTH as f32
            || e.transform.position.y > WINDOW_HEIGHT as f32
        {
            e.destroy();
        }
    }
}

fn move_entities(entities: &mut Vec<Entity>, dt: f32) {
    for e in entities.iter_mut() {
        if e.is_alive() {
            e.transform.position += e.transform.velocity * dt;
        }
    }
}

fn render(entities: &Vec<Entity>, draw_thread: &mut RaylibDrawHandle) {
    for e in entities.iter() {
        if e.is_alive() {
            let position = e.transform.position;
            draw_thread.draw_circle(position.x as i32, position.y as i32, 5.0, Color::WHITE);
        }
    }
}

fn shoot(entity_manager: &mut EntityManager, cd: &mut f32, offset: f32) {
    if *cd > 0.0 {
        return;
    }
    *cd = 0.2;

    let center = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
    let count = 20;
    let angle = 6.28 / count as f32;
    let mut theta: f32 = 0.0;
    for _ in 0..count {
        let velocity = Vec2::new(f32::cos(theta + offset), f32::sin(theta + offset));

        let e = entity_manager.add_entity("ball".to_string());
        e.transform = CTransform {
            position: velocity + center,
            velocity: velocity * 200.0,
            rotation: 0.0,
        };
        theta += angle;
    }
}
