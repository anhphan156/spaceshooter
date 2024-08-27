use crate::util::geometry::Shape;

#[derive(Clone, Debug)]
pub struct CBBox {
    pub shape: Shape,
    pub overlapped_shape: (f32, f32),
    pub collision_axes: (bool, bool),
}

impl Default for CBBox {
    fn default() -> Self {
        CBBox {
            shape: Shape::Rectangle(1.0, 1.0),
            overlapped_shape: (0.0, 0.0),
            collision_axes: (false, false),
        }
    }
}
