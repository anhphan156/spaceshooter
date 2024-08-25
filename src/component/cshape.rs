use raylib::color::Color;

use crate::util::geometry::Shape;

#[derive(Clone)]
pub struct CShape {
    pub color: Color,
    pub shape: Shape,
}

impl Default for CShape {
    fn default() -> Self {
        CShape {
            color: Color::RED,
            shape: Shape::Circle(10.0),
        }
    }
}
