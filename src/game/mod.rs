use crate::{
    component::ctransform::CTransform,
    entity::entity_manager::EntityManager,
    system,
    util::constant::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
};
use glam::Vec2;
use raylib::prelude::*;

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    entity_manager: EntityManager,
}

impl Game {
    pub fn new() -> Game {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .title(WINDOW_TITLE)
            .build();

        rl.set_target_fps(120);

        let mut entity_manager = EntityManager::new();
        for i in 0..10 {
            let e = entity_manager.add_entity("ball".to_string());
            e.transform = CTransform {
                position: Vec2::new(
                    WINDOW_WIDTH as f32 / 2.0 + (i as f32 * 70.0),
                    WINDOW_HEIGHT as f32 / 2.0,
                ),
                velocity: Vec2::new(0.0, i as f32 * 10.0),
                rotation: 0.0,
            };
        }

        Game {
            rl,
            thread,
            entity_manager,
        }
    }

    pub fn run(&mut self) {
        let center = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);

            d.clear_background(Color::WHITE);
            d.draw_fps(12, 12);

            self.entity_manager.update();

            if let Some(entities) = self.entity_manager.get_entities(None) {
                system::movement::move_entities(entities, d.get_frame_time());
                system::collision::check_out_of_bound(entities);
                system::render::render(entities, &mut d);
                d.draw_text(
                    format!("W: {} - H: {}", WINDOW_WIDTH, WINDOW_HEIGHT).as_str(),
                    0,
                    center.0,
                    30,
                    Color::BLACK,
                );
                d.draw_text(
                    format!("Entity Count: {}", self.entity_manager.count()).as_str(),
                    center.0,
                    0,
                    30,
                    Color::BLACK,
                );
                d.draw_line(center.0, 0, center.0, WINDOW_HEIGHT as i32, Color::RED);
                d.draw_line(0, center.1, WINDOW_WIDTH as i32, center.1, Color::RED);
            }
        }
    }
}
