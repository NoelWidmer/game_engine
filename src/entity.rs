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
    
    pub fn add_component<C: Any>(&mut self, component: C) -> Result<(), ()> {
        let type_id = TypeId::of::<C>();

        if self.components.insert(type_id, Box::new(component)).is_some() {
            Err(())
        } else {
            Ok(())
        }
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

    pub fn remove_component<C: Any>(&mut self) {
        let type_id = TypeId::of::<C>();
        self.components.remove(&type_id);
    }

    const ERR: &'static str = "a component is expected to be of the type it was registered as";
}