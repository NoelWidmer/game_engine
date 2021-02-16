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

    pub fn component_kinds(&self) -> Vec<&TypeId> {
        self
            .components
            .keys()
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

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use super::Entity;

    #[derive(Debug)]
    struct DummyComponent1 { }
    
    #[derive(Debug)]
    struct DummyComponent2 { }

    #[derive(Debug)]
    struct DummyComponent3 { 
        text: String
    }

    #[test]
    fn ensure_entity_is_created_without_components() {
        let entity = Entity::new();
        assert_eq!(entity.component_kinds().len(), 0);
    }

    #[test]
    fn ensure_entity_can_store_only_one_component_of_same_type() {
        let mut entity = Entity::new();

        let res1 = entity.add_component(DummyComponent1 { });
        assert!(res1.is_ok());
        assert_eq!(entity.component_kinds().len(), 1);

        let res1 = entity.add_component(DummyComponent1 { });
        assert!(res1.is_err());
        assert_eq!(entity.component_kinds().len(), 1);
    }

    #[test]
    fn ensure_entity_can_store_multiple_components() {
        let mut entity = Entity::new();

        let res1 = entity.add_component(DummyComponent1 { });
        assert!(res1.is_ok());
        assert_eq!(entity.component_kinds().len(), 1);

        let res1 = entity.add_component(DummyComponent2 { });
        assert!(res1.is_ok());
        assert_eq!(entity.component_kinds().len(), 2);
    }

    #[test]
    fn ensure_component_kinds_can_be_retrieved() {
        let mut entity = Entity::new();

        entity.add_component(DummyComponent1 { }).unwrap();
        entity.add_component(DummyComponent2 { }).unwrap();

        let kinds = entity.component_kinds();

        assert_eq!(kinds.len(), 2);
        assert!(kinds.contains(&&TypeId::of::<DummyComponent1>()));
        assert!(kinds.contains(&&TypeId::of::<DummyComponent2>()));
    }

    #[test]
    fn ensure_existing_component_can_retrieved() {
        let mut entity = Entity::new();

        entity.add_component(DummyComponent3 { text: "test".to_string() }).unwrap();
        let component = entity.component::<DummyComponent3>();
        
        assert!(component.is_some());
        assert_eq!(component.unwrap().text, "test");
    }

    #[test]
    fn ensure_non_existing_component_can_not_be_retrieved() {
        let entity = Entity::new();
        let component = entity.component::<DummyComponent1>();        
        assert!(component.is_none());
    }

    #[test]
    fn ensure_existing_component_can_retrieved_mutably() {
        let mut entity = Entity::new();

        entity.add_component(DummyComponent3 { text: "test".to_string() }).unwrap();
        let component = entity.component_mut::<DummyComponent3>();
        
        assert!(component.is_some());
        assert_eq!(component.unwrap().text, "test");
    }

    #[test]
    fn ensure_non_existing_component_can_not_be_retrieved_mutably() {
        let mut entity = Entity::new();
        let component = entity.component_mut::<DummyComponent1>();        
        assert!(component.is_none());
    }

    #[test]
    fn ensure_components_can_be_removed() {
        let mut entity = Entity::new();

        entity.add_component(DummyComponent1 { }).unwrap();
        entity.add_component(DummyComponent2 { }).unwrap();

        entity.remove_component::<DummyComponent1>();
        assert_eq!(entity.component_kinds().len(), 1);

        entity.remove_component::<DummyComponent2>();
        assert_eq!(entity.component_kinds().len(), 0);
    }
}