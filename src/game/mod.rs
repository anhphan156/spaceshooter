use ::core::panic;
use std::collections::HashMap;

use crate::{
    entity::entity_manager::EntityManager,
    scene::{mario::MarioScene, Scene},
    system,
    util::constant::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
};
use raylib::prelude::*;

pub struct Game<'a> {
    rl: RaylibHandle,
    thread: RaylibThread,
    scenes: HashMap<u8, Box<dyn Scene>>,
    current_scene: &'a Box<dyn Scene>,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .title(WINDOW_TITLE)
            .build();

        rl.set_target_fps(120);

        let mario_scene = MarioScene::new();
        let mario_scene = Box::new(mario_scene);

        let mut scenes: HashMap<u8, Box<dyn Scene>> = HashMap::new();
        scenes.insert(0, mario_scene);

        let current_scene = scenes.get(&0).unwrap();

        Game {
            rl,
            thread,
            scenes,
            current_scene,
        }
    }

    pub fn run(&mut self) {
        let center = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);
        //let mut cd: f32 = 0.0;
        //let mut offset: f32 = 0.0;
        while !self.rl.window_should_close() {
            //cd -= self.rl.get_frame_time();
            //offset -= 1.50;
            let mut d = self.rl.begin_drawing(&self.thread);

            d.clear_background(Color::BLACK);
            d.draw_fps(12, 12);

            //self.entity_manager.update();
            //system::shoot::shoot(&mut self.entity_manager, &mut cd, offset);
            //
            //if let Some(entities) = self.entity_manager.get_entities(None) {
            //    system::movement::move_entities(entities, d.get_frame_time());
            //    system::collision::check_out_of_bound(entities);
            //    system::render::render(entities, &mut d);
            //}
            //
            //d.draw_text(
            //    format!("Entity Count: {}", self.entity_manager.count()).as_str(),
            //    center.0,
            //    0,
            //    30,
            //    Color::RED,
            //);
            d.draw_line(center.0, 0, center.0, WINDOW_HEIGHT as i32, Color::RED);
            d.draw_line(0, center.1, WINDOW_WIDTH as i32, center.1, Color::RED);
        }
    }
}
