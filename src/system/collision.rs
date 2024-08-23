use crate::{
    entity::Entity,
    util::constant::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub fn check_out_of_bound(entities: &mut Vec<Entity>) {
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
