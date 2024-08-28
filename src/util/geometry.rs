#[derive(Clone, Debug)]
pub enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    RectText(f32, f32, f32, f32, &'static str),
}
