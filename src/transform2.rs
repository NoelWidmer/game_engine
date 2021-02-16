use std::collections::HashSet;
use super::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Transform2 {
    parent_entity_id: Option<u64>,
    child_entity_ids: HashSet<u64>,
    abs_location: Vec2,
    location: Vec2
}


impl Default for Transform2 {
    fn default() -> Self {
        Self::new_orphan()
    }
}

impl Transform2 {
    pub fn new_orphan() -> Self {
        Self {
            parent_entity_id: None,
            child_entity_ids: HashSet::new(),
            abs_location: Vec2::default(),
            location: Vec2::default()
        }
    }

    pub fn parent_entity_id(&self) -> Option<u64> {
        self.parent_entity_id
    } 

    pub fn parent_location(&self) -> Option<Vec2> {
        self.parent_entity_id.map(|_| self.abs_location - self.location)
    }

    pub fn set_parent(&mut self, parent_entity_id: u64, parent_transform: &Transform2) {
        self.parent_entity_id = Some(parent_entity_id);
        self.set_parent_transform(parent_transform);
    }

    pub fn set_parent_transform(&mut self, parent_transform: &Transform2) {
        self.abs_location = parent_transform.abs_location() + self.location;
    }

    pub fn add_child(&mut self, child_entity_id: u64) {
        self.child_entity_ids.insert(child_entity_id);
    }

    pub fn remove_child(&mut self, child_entity_id: &u64) {
        self.child_entity_ids.remove(&child_entity_id);
    }

    pub fn abs_location(&self) -> Vec2 {
        self.abs_location
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }

    pub fn set_location(&mut self, location: Vec2) {
        self.add_location(location - self.location);
    }

    pub fn add_location(&mut self, location: Vec2)  {
        self.abs_location += location;
        self.location += location;
    }
}

/*#[cfg(test)]
mod tests {
    use super::super::vec2::Vec2;
    use super::Transform2;

    #[test]
    fn new_orphan_is_at_world_origin() {
        let orphan = Transform2::new_orphan();
        assert_eq!(orphan.parent_location(), None);
        assert_eq!(orphan.abs_location(), Vec2::default());
        assert_eq!(orphan.location(), Vec2::default());
    }

    #[test]
    fn orphan_has_no_parent() {
        let orphan = Transform2::new_orphan();
        assert_eq!(orphan.parent_entity_id(), None);
        assert_eq!(orphan.parent_location(), None);
    }

    #[test]
    fn child_has_parent() {
        let parent = Transform2::new_orphan();
        let parent_location = Vec2::new(3f32, 6f32);
        parent.set_location(parent_location);

        let orphan = Transform2::new_orphan();
        orphan.set_parent(42, &parent);
        let child = orphan;

        assert_eq!(child.parent_entity_id(), Some(42));
        assert_eq!(child.parent_location(), Some(parent_location));
    }

    #[test]
    fn new_child_is_at_parent_origin() {
        let mut parent = Transform2::new_orphan();
        let parent_location = Vec2::new(3f32, 6f32);
        parent.set_location(parent_location);

        let child = Transform2::new_child(0, &parent);

        assert_eq!(child.parent_location(), Some(parent_location));
        assert_eq!(child.abs_location(), parent_location);
        assert_eq!(child.location(), Vec2::default());
    }

    #[test]
    fn locations_are_calculated_correctly() {
        let mut parent = Transform2::new_orphan();
        let parent_location = Vec2::new(3f32, 6f32);
        parent.set_location(parent_location);

        let mut child = Transform2::new_child(0, &parent);
        let child_location = Vec2::new(1f32, 2f32);
        child.set_location(child_location);

        assert_eq!(child.parent_location(), Some(parent_location));
        assert_eq!(child.abs_location(), parent_location + child_location);
        assert_eq!(child.location(), child_location);
    }

    #[test]
    fn set_parent_location() {
        let parent_location = Vec2::new(3f32, 6f32);
        let child_location = Vec2::new(1f32, 2f32);

        let mut child = Transform2::new_orphan();
        child.set_parent_location(parent_location);
        child.set_location(child_location);

        assert_eq!(child.parent_location(), parent_location);
        assert_eq!(child.abs_location(), parent_location + child_location);
        assert_eq!(child.location(), child_location);
    }

    #[test]
    fn add_location() {
        let parent_location = Vec2::new(3f32, 6f32);
        let child_location = Vec2::new(1f32, 2f32);
        let location_to_add = Vec2::new(1f32, 1f32);

        let mut child = Transform2::new_orphan();
        child.set_parent_location(parent_location);
        child.set_location(child_location);

        child.add_location(location_to_add);

        assert_eq!(child.parent_location(), parent_location);
        assert_eq!(child.abs_location(), parent_location + child_location + location_to_add);
        assert_eq!(child.location(), child_location + location_to_add);
    }
}*/