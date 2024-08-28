#[derive(Clone, Debug)]
pub struct Animation {
    pub anim_frame: u32,
    game_frame: u32,
    anim_speed: u32,
    anim_frame_count: u32,
}

impl Animation {
    pub fn new(anim_speed: u32, anim_frame_count: u32) -> Animation {
        Animation {
            game_frame: 0,
            anim_frame: 0,
            anim_speed,
            anim_frame_count,
        }
    }

    pub fn update(&mut self) {
        self.game_frame += 1;
        self.anim_frame = self.game_frame / self.anim_speed % self.anim_frame_count;
    }
}
