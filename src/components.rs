use std::any::{ Any, TypeId };
use std::collections::{ HashMap, hash_map::Entry };

pub struct Components {
    map: HashMap<TypeId, Box<dyn Any>>
}

impl Components {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity)
        }
    }

    pub fn add_default<C: Any + Default>(self) -> Result<Self, ()> {
        self.add::<C>(C::default())
    }

    pub fn add<C: Any>(mut self, component: C) -> Result<Self, ()> {
        let type_id = TypeId::of::<C>();

        match self.map.entry(type_id) {
            Entry::Occupied(_) => Err(()), 
            Entry::Vacant(entry) => {
                entry.insert(Box::new(component));
                Ok(self)
            }
        }
    }

    pub fn consume(self) -> HashMap<TypeId, Box<dyn Any>> {
        self.map
    }
}