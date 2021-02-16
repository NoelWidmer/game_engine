use std::{
    any::{ 
        Any, 
        TypeId
    }, 
    collections::{ 
        HashMap, 
        hash_map::Entry,
        HashSet 
    }
};
use super::{
    entity::Entity
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

    pub fn spawn_entity(&mut self) -> u64 {
        let entity_id = self.next_entity_id;
        self.next_entity_id = entity_id + 1;

        if self.entities.insert(entity_id, Entity::new()).is_some() {
            panic!("could not spawn entity with id {} because that id is already in use.", entity_id);
        }

        entity_id
    }

    pub fn despawn_entity(&mut self, entity_id: &u64) {
        if let Some(entity) = self.entities.remove(entity_id) {
            for component_kind in entity.component_kinds() {
                self
                    .component_registry
                    .entry(component_kind)
                    .and_modify(|entry| {
                        entry.remove(entity_id);
                    });
            }
        }
    }   

    pub fn add_default_component<C: Any + Default>(&mut self, entity_id: u64) -> Result<(), ()> {
        self.add_component::<C>(entity_id, C::default())
    }

    pub fn add_component<C: Any>(&mut self, entity_id: u64, component: C) -> Result<(), ()> {
        let type_id = TypeId::of::<C>();

        if let Some(entity) = self.entities.get_mut(&entity_id) {
            if entity.add_component(component).is_ok() {
                match self.component_registry.entry(type_id) {
                    Entry::Occupied(mut entry) => {
                        if entry.get_mut().insert(entity_id) == false {
                            panic!("entity with id {} was not expected to have component of type {:?}", entity_id, type_id);
                        }
                    }, 
                    Entry::Vacant(entry) => {
                        let mut set = HashSet::with_capacity(1);
                        set.insert(entity_id);
                        entry.insert(set);
                    }
                }
                
                Ok(())
            } else {
                Err(()) // entity already contains component
            }
        } else {
            Err(()) // entity does not exist
        }
    }

    pub fn get_component<C: Any>(&self, entity_id: u64) -> Option<&C> {
        self
            .entities
            .get(&entity_id)
            .map(|entity| entity.component::<C>())
            .flatten()
    }

    pub fn component_mut<C: Any>(&mut self, entity_id: u64) -> Option<&mut C> {
        self
            .entities
            .get_mut(&entity_id)
            .map(|entity| entity.component_mut::<C>())
            .flatten()
    }

    pub fn remove_component<C: Any>(&mut self, entity_id: u64) {
        self
            .entities
            .get_mut(&entity_id)
            .map(|entity| entity.remove_component::<C>());
    }

    pub fn find_entities_with_component<C: Any>(&self) -> HashSet<u64> {
        let type_id = TypeId::of::<C>();
            
        match self.component_registry.get(&type_id) {
            Some(entity_ids) => entity_ids.clone(), 
            None => HashSet::new()
        }
    }
}