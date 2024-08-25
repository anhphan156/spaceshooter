use crate::component::{cshape::CShape, ctransform::CTransform};

pub mod entity_manager;

#[allow(unused)]
#[derive(Clone)]
pub struct Entity {
    is_alive: bool,
    id: u64,
    tag: String,
    pub c_transform: CTransform,
    pub c_shape: CShape,
}

#[allow(unused)]
impl Entity {
    fn new() -> Entity {
        Entity {
            is_alive: true,
            id: 0,
            tag: String::new(),
            c_transform: CTransform::new(),
            c_shape: CShape::default(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn destroy(&mut self) {
        self.is_alive = false
    }
}
