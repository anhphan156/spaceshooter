use crate::entity::Entity;

pub fn move_entities(entities: &mut Vec<Entity>, dt: f32) {
    for e in entities.iter_mut() {
        if e.is_alive() {
            e.transform.position += e.transform.velocity * dt;
        }
    }
}
