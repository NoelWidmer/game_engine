use super::{
    world::World, 
    vec2::Vec2, 
    transform2::Transform2
};

pub struct Transformer2 { }

pub enum TransformerError {
    EntityDoesNotExist, 
    EntityDoesNotHaveTransform2
}

impl Transformer2 {
    pub fn set_location(world: &mut World, entity_id: u64, location: Vec2) -> Result<(), TransformerError> {
        Self::find_transform2(world, entity_id).map(|t| t.set_location(location))
    }

    pub fn add_location(world: &mut World, entity_id: u64, location: Vec2) -> Result<(), TransformerError> {
        Self::find_transform2(world, entity_id).map(|t| t.add_location(location))
    }

    fn find_transform2(world: &mut World, entity_id: u64) -> Result<&mut Transform2, TransformerError> {
        match world.entity_mut(&entity_id) {
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