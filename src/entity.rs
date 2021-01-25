use std::{
    any::{ 
        Any, 
        TypeId 
    }, 
    collections::HashMap
};
use super::components::Components;

pub struct Entity {
    components: HashMap<TypeId, Box<dyn Any>>
}

impl Entity {
    pub fn new(components: Components) -> Self {
        Self {
            components: components.consume()
        }
    }

    pub fn component_kinds_ref(&self) -> Vec<&TypeId> {
        self
            .components
            .keys()
            .collect()
    }

    pub fn component_kinds(self) -> Vec<TypeId> {
        self
            .components
            .into_iter()
            .map(|(k, _)| k)
            .collect()
    }

    pub fn component<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let any = self.components.get(&type_id);
        any.map(|c| c.downcast_ref().expect(Self::err()))
    }

    pub fn component_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let any = self.components.get_mut(&type_id);
        any.map(|c| c.downcast_mut().expect(Self::err()))
    }

    fn err() -> &'static str {
        "a component is expected to be of the type it was registered as"
    }
}