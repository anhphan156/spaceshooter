use std::{collections::HashMap, rc::Rc};

use crate::{
    asset::AssetManager,
    scene::{mario::MarioScene, Scene},
    util::constant::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH},
};
use raylib::prelude::*;

pub struct Game<'a> {
    rl: RaylibHandle,
    thread: RaylibThread,
    scenes: HashMap<u8, Box<dyn Scene>>,
    current_scene: Option<&'a mut Box<dyn Scene>>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        let (mut rl, thread) = raylib::init()
            .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .title(WINDOW_TITLE)
            .build();

        rl.set_target_fps(120);

        let asset_manager = Rc::new(AssetManager::new(&mut rl, &thread));

        let mario_scene = MarioScene::new(Rc::clone(&asset_manager));
        let mario_scene = Box::new(mario_scene);

        let mut scenes: HashMap<u8, Box<dyn Scene>> = HashMap::new();
        scenes.insert(0, mario_scene);

        Game {
            rl,
            thread,
            scenes,
            current_scene: None,
        }
    }

    #[allow(dead_code)]
    fn change_scene(&'a mut self, name: u8) {
        self.current_scene = self.scenes.get_mut(&name);
    }

    pub fn run(&'a mut self) {
        self.current_scene = self.scenes.get_mut(&0);

        while !self.rl.window_should_close() {
            if let Some(scene) = &mut self.current_scene {
                scene.update(&mut self.rl, &mut self.thread);
            } else {
                panic!("No scene selected");
            }
        }
    }
}
