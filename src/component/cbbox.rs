use crate::util::geometry::Shape;

#[derive(Clone)]
pub struct CBBox {
    pub shape: Shape,
}

impl Default for CBBox {
    fn default() -> Self {
        CBBox {
            shape: Shape::Rectangle(1.0, 1.0),
        }
    }
}
