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
    let a_top = a_pos.y - a_shape.y;
    let a_bottom = a_pos.y + a_shape.y;
    let a_left = a_pos.x - a_shape.x;
    let a_right = a_pos.x + a_shape.x;

    let b_top = b_pos.y - b_shape.y;
    let b_bottom = b_pos.y + b_shape.y;
    let b_left = b_pos.x - b_shape.x;
    let b_right = b_pos.x + b_shape.x;

    let h_collision = a_top < b_bottom && b_top < a_bottom;
    let v_collision = a_left < b_right && b_left < a_right;

    let dx = if f32::abs(a_left - b_right) > f32::abs(b_left - a_right) {
        f32::abs(b_left - a_right)
    } else {
        f32::abs(a_left - b_right)
    };

    let dy = if f32::abs(a_top - b_bottom) > f32::abs(b_top - a_bottom) {
        f32::abs(b_top - a_bottom)
    } else {
        f32::abs(a_top - b_bottom)
    };

    AABBCollisionResult {
        collision_axes: (h_collision, v_collision),
        overlapped_shape: (dx, dy),
    }
}
