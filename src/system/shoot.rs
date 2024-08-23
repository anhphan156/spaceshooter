use glam::Vec2;

use crate::{
    component::ctransform::CTransform,
    entity::entity_manager::EntityManager,
    util::constant::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub fn shoot(entity_manager: &mut EntityManager, cd: &mut f32, offset: f32) {
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
