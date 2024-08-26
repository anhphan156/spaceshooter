use std::ops::Deref;

use crate::component::{cbbox::CBBox, cinput::CInput, cshape::CShape, ctransform::CTransform};

pub mod entity_manager;

#[allow(unused)]
#[derive(Clone)]
pub struct Entity {
    is_alive: bool,
    id: u64,
    tag: String,
    pub c_transform: CTransform,
    pub c_shape: CShape,
    pub c_bbox: CBBox,
    pub c_input: CInput,
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
            c_bbox: CBBox::default(),
            c_input: CInput::default(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn destroy(&mut self) {
        self.is_alive = false
    }
}
