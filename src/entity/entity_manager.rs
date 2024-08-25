use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::component::{cshape::CShape, ctransform::CTransform};

use super::Entity;

pub struct EntityManager {
    entity_count: u64,
    pending: Vec<Rc<RefCell<Entity>>>,
    entities: Vec<Rc<RefCell<Entity>>>,
    tagged_entities: HashMap<String, Vec<Rc<RefCell<Entity>>>>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entity_count: 0,
            pending: vec![],
            entities: vec![],
            tagged_entities: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for e in self.pending.iter_mut() {
            self.entities.push(Rc::clone(e));

            if let Some(entities) = self.tagged_entities.get_mut(&e.borrow().tag) {
                entities.push(Rc::clone(e));
            } else {
                self.tagged_entities
                    .insert(e.borrow().tag.clone(), vec![Rc::clone(e)]);
            };
        }
        self.pending.clear();

        for e in self.entities.iter() {
            if !e.borrow().is_alive {
                if let Some(tagged_e) = self.tagged_entities.get_mut(&e.borrow().tag) {
                    tagged_e.retain(|x| x.borrow().is_alive)
                }
            }
        }
        self.entities.retain(|x| x.borrow().is_alive);
    }

    pub fn count(&self) -> u64 {
        self.entities.len() as u64
    }

    pub fn get_entities(&mut self, tag: Option<String>) -> Option<&mut Vec<Rc<RefCell<Entity>>>> {
        match tag {
            Some(t) => self.tagged_entities.get_mut(&t),
            _ => Some(&mut self.entities),
        }
    }

    pub fn add_entity(&mut self, tag: String) -> Rc<RefCell<Entity>> {
        let e: Entity = Entity {
            is_alive: true,
            id: self.entity_count,
            tag,
            c_transform: CTransform::new(),
            c_shape: CShape::default(),
        };
        let e = Rc::new(RefCell::new(e));
        self.pending.push(e);

        self.entity_count += 1;

        self.pending.last().unwrap().clone()
    }
}
