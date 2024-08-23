use glam::Vec2;

#[derive(Clone)]
pub struct CTransform {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
}

impl CTransform {
    pub fn new() -> CTransform {
        CTransform {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            rotation: 0.0_f32,
        }
    }
}
