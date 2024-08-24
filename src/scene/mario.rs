use glam::Vec2;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    component::ctransform::CTransform,
    entity::{entity_manager::EntityManager, Entity},
    util::constant::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

use super::Scene;

pub struct MarioScene {
    pub entity_manager: EntityManager,
    center: (i32, i32),
    offset: f32,
    cd: f32,
}

impl MarioScene {
    pub fn new() -> MarioScene {
        let entity_manager = EntityManager::new();
        let center = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);

        MarioScene {
            entity_manager,
            center,
            offset: 0.0,
            cd: 0.0,
        }
    }

    fn draw_axes(&self, d: &mut RaylibDrawHandle) {
        d.draw_line(
            self.center.0,
            0,
            self.center.0,
            WINDOW_HEIGHT as i32,
            Color::RED,
        );
        d.draw_line(
            0,
            self.center.1,
            WINDOW_WIDTH as i32,
            self.center.1,
            Color::RED,
        );
    }

    fn move_entities(entities: &mut Vec<Entity>, dt: f32) {
        for e in entities.iter_mut() {
            if e.is_alive() {
                e.transform.position += e.transform.velocity * dt;
            }
        }
    }
    fn render(entities: &Vec<Entity>, d: &mut RaylibDrawHandle) {
        for e in entities.iter() {
            if e.is_alive() {
                let position = e.transform.position;
                d.draw_circle(position.x as i32, position.y as i32, 5.0, Color::WHITE);
            }
        }
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
    fn shoot(&mut self, dt: f32) {
        self.offset += 1.0;
        if self.cd > 0.0 {
            self.cd -= dt;
            return;
        }
        self.cd = 0.2;

        let center = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
        let count = 20;
        let angle = 6.28 / count as f32;
        let mut theta: f32 = 0.0;
        for _ in 0..count {
            let velocity = Vec2::new(f32::cos(theta + self.offset), f32::sin(theta + self.offset));

            let e = self.entity_manager.add_entity("ball".to_string());
            e.transform = CTransform {
                position: velocity + center,
                velocity: velocity * 200.0,
                rotation: 0.0,
            };
            theta += angle;
        }
    }
}

impl Scene for MarioScene {
    fn update(&mut self, d: &mut RaylibDrawHandle, dt: f32) {
        d.clear_background(Color::BLACK);
        d.draw_fps(12, 12);

        self.draw_axes(d);

        self.entity_manager.update();

        self.shoot(dt);
        if let Some(entities) = self.entity_manager.get_entities(None) {
            MarioScene::move_entities(entities, dt);
            MarioScene::check_out_of_bound(entities);
            MarioScene::render(entities, d);
            d.draw_text(
                format!("{}", entities.len()).as_str(),
                self.center.0,
                0,
                30,
                Color::RED,
            );
        }
    }
}
