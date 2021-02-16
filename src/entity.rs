use std::{
    any::{ 
        Any, 
        TypeId 
    }, 
    collections::HashMap
};

pub struct Entity {
    components: HashMap<TypeId, Box<dyn Any>>
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }
    
    /*pub fn new_with_components(components: Components) -> Self {
        Self {
            components: components.consume()
        }
    }*/

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

    pub fn component<C: Any>(&self) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        let any = self.components.get(&type_id);
        any.map(|c| c.downcast_ref().expect(Self::ERR))
    }

    pub fn component_mut<C: Any>(&mut self) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        let any = self.components.get_mut(&type_id);
        any.map(|c| c.downcast_mut().expect(Self::ERR))
    }

    const ERR: &'static str = "a component is expected to be of the type it was registered as";
}