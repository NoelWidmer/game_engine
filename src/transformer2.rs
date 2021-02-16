use super::{
    world::World, 
    vec2::Vec2, 
    transform2::Transform2
};

pub struct Transformer2 { }

#[derive(Debug)]
pub enum TransformerError {
    EntityDoesNotExist, 
    EntityDoesNotHaveTransform2
}

impl Transformer2 {
    pub fn parent(world: &mut World, parent_entity_id: u64, child_entity_id: u64) -> Result<(), TransformerError> {
        let res = Self::find_transform2(world, parent_entity_id) 
            .map(|parent_transform| {
                parent_transform.add_child(child_entity_id);
                parent_transform.clone()
            });

        match res {
            Ok(parent_transform) => {
                Self::find_transform2(world, child_entity_id)
                    .map(|child_transform| child_transform.set_parent(parent_entity_id, &parent_transform))
            }, 
            Err(err) => Err(err)
        }
    }

    pub fn set_location(world: &mut World, entity_id: u64, location: Vec2) -> Result<(), TransformerError> {
        Self::find_transform2(world, entity_id)
            .map(|t| t.set_location(location))
    }

    pub fn add_location(world: &mut World, entity_id: u64, location: Vec2) -> Result<(), TransformerError> {
        Self::find_transform2(world, entity_id)
            .map(|t| t.add_location(location))
    }

    fn find_transform2(world: &mut World, entity_id: u64) -> Result<&mut Transform2, TransformerError> {
        match world.entity_mut(entity_id) {
            Some(entity) => {
                match entity.component_mut::<Transform2>() {
                    Some(component) => Ok(component), 
                    None => Err(TransformerError::EntityDoesNotHaveTransform2)
                }
            }, 
            None => Err(TransformerError::EntityDoesNotExist)
        }
    }
}