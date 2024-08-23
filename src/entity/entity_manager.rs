use std::collections::HashMap;

use crate::component::ctransform::CTransform;

use super::Entity;

pub struct EntityManager {
    entity_count: u64,
    pending: Vec<Entity>,
    entities: Vec<Entity>,
    tagged_entities: HashMap<String, Vec<Entity>>,
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
        for e in self.pending.iter() {
            self.entities.push(e.clone());

            let e = e.clone();
            if let Some(entities) = self.tagged_entities.get_mut(&e.tag) {
                entities.push(e);
            } else {
                self.tagged_entities.insert(e.tag.clone(), vec![e]);
            };
        }
        self.pending.clear();

        for e in self.entities.iter() {
            if !e.is_alive {
                if let Some(tagged_e) = self.tagged_entities.get_mut(&e.tag) {
                    tagged_e.retain(|x| x.is_alive)
                }
            }
        }
        self.entities.retain(|x| x.is_alive);
    }

    pub fn count(&self) -> u64 {
        self.entities.len() as u64
    }

    pub fn get_entities(&mut self, tag: Option<String>) -> Option<&mut Vec<Entity>> {
        match tag {
            Some(t) => self.tagged_entities.get_mut(&t),
            _ => Some(&mut self.entities),
        }
    }

    pub fn add_entity(&mut self, tag: String) -> &mut Entity {
        let e: Entity = Entity {
            is_alive: true,
            id: self.entity_count,
            tag,
            transform: CTransform::new(),
        };
        self.pending.push(e);

        self.entity_count += 1;

        self.pending.last_mut().unwrap()
    }
}
