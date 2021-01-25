use std::{
    any::{ 
        Any, 
        TypeId
    }, 
    collections::{ 
        HashMap, 
        HashSet 
    }
};
use super::{
    entity::Entity, 
    components::Components
};

pub struct World {
    next_entity_id: u64,
    entities: HashMap<u64, Entity>,
    component_registry: HashMap<TypeId, HashSet<u64>>
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entities: HashMap::new(), 
            component_registry: HashMap::new()
        }
    }

    pub fn spawn_entity(&mut self, components: Components) -> u64 {
        let entity_id = self.next_entity_id;
        self.next_entity_id = entity_id + 1;

        let entity = Entity::new(components);

        for component_kind in entity.component_kinds_ref() {
            self
                .component_registry
                .entry(component_kind.clone())
                .or_insert(HashSet::new())
                .insert(entity_id);
        }

        if self.entities.insert(entity_id, entity).is_some() {
            panic!("could not spawn entity with id {} because that id is already in use.", entity_id);
        }

        entity_id
    }

    pub fn despawn_entity(&mut self, entity_id: &u64) {
        match self.entities.remove(entity_id) {
            Some(entity) => {
                for component_kind in entity.component_kinds() {
                    self
                        .component_registry
                        .entry(component_kind)
                        .or_insert(HashSet::new())
                        .remove(entity_id);
                }
            }, 
            None => ()
        }
    }

    pub fn find_entities_with_component<C: Any>(&self) -> HashSet<u64> {
        let type_id = TypeId::of::<C>();
            
        match self.component_registry.get(&type_id) {
            Some(entity_ids) => entity_ids.clone(), 
            None => HashSet::new()
        }
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn entity(&self, entity_id: &u64) -> Option<&Entity> {
        self.entities.get(entity_id)
    }

    pub fn entity_mut(&mut self, entity_id: &u64) -> Option<&mut Entity> {
        self.entities.get_mut(entity_id)
    }
}