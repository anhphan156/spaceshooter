use crate::component::ctransform::CTransform;

pub mod entity_manager;

enum Tag {
    Recyclable(String),
    UnRecyclable(String),
}

#[allow(unused)]
#[derive(Clone)]
pub struct Entity {
    is_alive: bool,
    id: u64,
    tag: String,
    pub transform: CTransform,
}

#[allow(unused)]
impl Entity {
    fn new() -> Entity {
        Entity {
            is_alive: true,
            id: 0,
            tag: String::new(),
            transform: CTransform::new(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn destroy(&mut self) {
        self.is_alive = false
    }
}
