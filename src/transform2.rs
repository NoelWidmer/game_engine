use super::vec2::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Transform2 {
    location: Vec2
}


impl Default for Transform2 {
    fn default() -> Self {
        Self {
            location: Vec2::default()
        }
    }
}

impl Transform2 {
    pub fn new(location: Vec2) -> Self {
        Self {
            location
        }
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }
    

    pub fn location_mut(&mut self) -> &mut Vec2 {
        &mut self.location
    }
}