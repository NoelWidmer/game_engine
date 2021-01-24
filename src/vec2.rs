#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32, 
    pub y: f32
}

impl Default for Vec2 {
    fn default() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}