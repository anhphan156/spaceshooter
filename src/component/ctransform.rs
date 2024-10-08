use glam::Vec2;

#[derive(Clone, Debug)]
pub struct CTransform {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub rotation: f32,
    pub prev_position: Vec2,
}

impl Default for CTransform {
    fn default() -> Self {
        CTransform {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            rotation: 0.0_f32,
            prev_position: Vec2::new(0.0, 0.0),
        }
    }
}
