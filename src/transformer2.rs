use super::{
    world::World, 
    vec2::Vec2, 
    transform2::Transform2, 
    entity_id::EntityId
};
use std::collections::HashSet;

pub struct Transformer2 { }

impl Transformer2 {
    pub fn adopt(world: &mut World, parent_entity_id: EntityId, child_entity_id: EntityId) -> Result<(), ()> {
        let parent_result = 
            if let Some(parent_transform) = world.component_mut::<Transform2>(parent_entity_id) {
                // attach parent to child
                parent_transform.add_child(child_entity_id);
                Ok(parent_transform.abs_location())
            } else {
                Err(())
            };

        if let Ok(parent_abs_location) = parent_result {
            if let Some(child_transform) = world.component_mut::<Transform2>(child_entity_id) {
                // attach child to parent
                child_transform.set_parent(parent_entity_id, parent_abs_location);
                Ok(())
            } else {
                // revert partially comitted adoption
                let parent_transform = world.component_mut::<Transform2>(parent_entity_id).unwrap();
                parent_transform.remove_child(&child_entity_id);
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn set_location(world: &mut World, entity_id: EntityId, location: Vec2) -> Result<(), ()> {
        world
            .component_mut::<Transform2>(entity_id)
            .map(|transform| {
                transform.set_location(location);
                (transform.abs_location(), transform.children_entity_ids().clone()) // todo optimize
            })
            .map(|data| Self::update_children_location(world, data.0, &data.1))
            .ok_or(())
    }

    pub fn add_location(world: &mut World, entity_id: EntityId, location: Vec2) -> Result<(), ()> {
        world
            .component_mut::<Transform2>(entity_id)
            .map(|transform| {
                transform.add_location(location);
                (transform.abs_location(), transform.children_entity_ids().clone()) // todo optimize
            })
            .map(|data| Self::update_children_location(world, data.0, &data.1))
            .ok_or(())
    }

    fn update_children_location(world: &mut World, parent_location: Vec2, children_entity_ids: &HashSet<EntityId>) {
        for child_entity_id in children_entity_ids {
            let data = {
                let child_transform = world.component_mut::<Transform2>(*child_entity_id).expect("");
                child_transform.set_parent_location(parent_location);
                (child_transform.abs_location(), child_transform.children_entity_ids().clone()) // todo optimize
            };

            Self::update_children_location(world, data.0, &data.1);
        }
    }
}