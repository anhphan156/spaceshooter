use std::collections::HashMap;

use crate::{
    scene::{mario::MarioScene, Scene},
    util::constant::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
};
use raylib::prelude::*;

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    scenes: HashMap<u8, Box<dyn Scene>>,
}

impl Game {
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

        Game { rl, thread, scenes }
    }

    pub fn run(&mut self) {
        let current_scene: &mut Box<dyn Scene> = self.scenes.get_mut(&0).unwrap();
        while !self.rl.window_should_close() {
            let dt = self.rl.get_frame_time();
            let mut d = self.rl.begin_drawing(&self.thread);
            current_scene.update(&mut d, dt);
        }
    }
}
