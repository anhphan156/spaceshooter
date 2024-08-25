use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    component::{cshape::CShape, ctransform::CTransform},
    entity::{entity_manager::EntityManager, Entity},
    util::{
        constant::{WINDOW_HEIGHT, WINDOW_WIDTH},
        geometry::Shape,
    },
};

use super::Scene;

#[allow(dead_code)]
pub struct MarioScene {
    pub entity_manager: EntityManager,
    player: Rc<RefCell<Entity>>,
    center: (i32, i32),
    offset: f32,
    cd: f32,
}

impl MarioScene {
    pub fn new() -> MarioScene {
        let mut entity_manager = EntityManager::new();
        let center = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);

        let player = MarioScene::spawn_player(&mut entity_manager);

        MarioScene {
            entity_manager,
            player,
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

    fn move_entities(entities: &mut Vec<Rc<RefCell<Entity>>>, dt: f32) {
        for e in entities.iter_mut() {
            let vec;
            {
                vec = e.borrow().c_transform.velocity;
            }
            if e.borrow().is_alive() {
                e.borrow_mut().c_transform.position += vec * dt;
            }
        }
    }
    fn render(entities: &Vec<Rc<RefCell<Entity>>>, d: &mut RaylibDrawHandle) {
        for e in entities.iter() {
            let e = e.borrow();
            if e.is_alive() {
                let position = e.c_transform.position;
                match e.c_shape.shape {
                    Shape::Circle(r) => {
                        d.draw_circle(position.x as i32, position.y as i32, r, e.c_shape.color)
                    }
                    Shape::Rectangle(w, h) => d.draw_rectangle(
                        position.x as i32 - w / 2,
                        position.y as i32 - h / 2,
                        w,
                        h,
                        e.c_shape.color,
                    ),
                }
            }
        }
    }
    fn check_out_of_bound(entities: &mut Vec<Rc<RefCell<Entity>>>) {
        for e in entities.iter_mut() {
            let mut e = e.borrow_mut();
            if !e.is_alive() {
                continue;
            }
            if e.c_transform.position.x < 0.0
                || e.c_transform.position.y < 0.0
                || e.c_transform.position.x > WINDOW_WIDTH as f32
                || e.c_transform.position.y > WINDOW_HEIGHT as f32
            {
                e.destroy();
            }
        }
    }

    #[allow(dead_code)]
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
            let mut e = e.borrow_mut();
            e.c_transform = CTransform {
                position: velocity + center,
                velocity: velocity * 200.0,
                rotation: 0.0,
            };
            e.c_shape = CShape {
                //shape: Shape::Circle(5.0),
                shape: Shape::Rectangle(25, 25),
                color: Color::WHITE,
            };
            theta += angle;
        }
    }

    fn spawn_player(entity_manager: &mut EntityManager) -> Rc<RefCell<Entity>> {
        let position = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);

        let player = entity_manager.add_entity("Player".to_string());
        {
            let mut p = player.borrow_mut();
            p.c_transform = CTransform {
                position,
                velocity: Vec2::new(0.0, 10.0),
                rotation: 0.0,
            };
            p.c_shape = CShape {
                shape: Shape::Rectangle(100, 100),
                color: Color::WHITE,
            };
        }

        player
    }
}

impl Scene for MarioScene {
    fn update(&mut self, d: &mut RaylibDrawHandle, dt: f32) {
        d.clear_background(Color::BLACK);
        d.draw_fps(12, 12);

        self.draw_axes(d);

        //self.shoot(dt);
        self.entity_manager.update();

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
