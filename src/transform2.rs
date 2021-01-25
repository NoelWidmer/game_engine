use super::vec2::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Transform2 {
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
            abs_location: Vec2::default(),
            location: Vec2::default()
        }
    }

    pub fn new_child(parent: Transform2) -> Self {
        Self {
            abs_location: parent.abs_location(),
            location: Vec2::default()
        }
    }

    pub fn parent_location(&self) -> Vec2 {
        self.abs_location - self.location
    }

    pub fn abs_location(&self) -> Vec2 {
        self.abs_location
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }

    pub fn set_parent_location(&mut self, location: Vec2) {
        self.abs_location = location + self.location;
    }

    pub fn set_location(&mut self, location: Vec2) {
        let vec = location - self.location;
        self.add_location(vec);
    }

    pub fn add_location(&mut self, location: Vec2)  {
        self.abs_location += location;
        self.location += location;
    }
}