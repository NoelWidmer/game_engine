use std::ops::{
    Add,
    AddAssign, 
    Sub, 
    SubAssign
};

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

    pub fn diff(self, other: Vec2) -> Vec2 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        Vec2::new(x_diff, y_diff)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}