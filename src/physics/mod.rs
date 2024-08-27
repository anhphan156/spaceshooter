use glam::Vec2;

pub struct AABBCollisionResult {
    pub collision_axes: (bool, bool),
    pub overlapped_shape: (f32, f32),
}

impl AABBCollisionResult {
    pub fn is_collided(&self) -> bool {
        self.collision_axes.0 && self.collision_axes.1
    }
}

pub fn aabb_collision_detection(
    a_pos: Vec2,
    b_pos: Vec2,
    a_shape: Vec2,
    b_shape: Vec2,
) -> AABBCollisionResult {
    let dx = f32::abs(a_pos.x - b_pos.x);
    let dy = f32::abs(a_pos.y - b_pos.y);

    let ox = a_shape.x + b_shape.x - dx;
    let oy = a_shape.y + b_shape.y - dy;

    AABBCollisionResult {
        collision_axes: (ox > 0.0, oy > 0.0),
        overlapped_shape: (ox, oy),
    }
}
