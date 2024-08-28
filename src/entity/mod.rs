use crate::component::{
    canimation::CAnimation, cbbox::CBBox, cinput::CInput, cshape::CShape, cstate::CState,
    ctransform::CTransform,
};

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
    pub c_state: CState,
    pub c_animation: CAnimation,
}

#[allow(unused)]
impl Entity {
    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn destroy(&mut self) {
        self.is_alive = false
    }

    pub fn is_collided(&self) -> bool {
        self.c_bbox.collision_axes.0 && self.c_bbox.collision_axes.1
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            is_alive: true,
            id: 0,
            tag: String::new(),
            c_transform: CTransform::default(),
            c_shape: CShape::default(),
            c_bbox: CBBox::default(),
            c_input: CInput::default(),
            c_state: CState::default(),
            c_animation: CAnimation::default(),
        }
    }
}
