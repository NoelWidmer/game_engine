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
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn eq() {
        let first = Vec2::new(1f32, 2f32);
        let second = Vec2::new(1f32, 2f32);
        assert_eq!(first, second)
    }

    #[test]
    fn neq_x() {
        let first = Vec2::new(1f32, 2f32);
        let second = Vec2::new(3f32, 2f32);
        assert_ne!(first, second)
    }

    #[test]
    fn neq_y() {
        let first = Vec2::new(1f32, 2f32);
        let second = Vec2::new(1f32, 3f32);
        assert_ne!(first, second)
    }

    #[test]
    fn add() {
        let sum = Vec2::new(1f32, 2f32) + Vec2::new(10f32, 20f32);
        assert_eq!(sum, Vec2::new(11f32, 22f32))
    }

    #[test]
    fn add_assign() {
        let mut sum = Vec2::new(1f32, 2f32);
        sum += Vec2::new(10f32, 20f32);
        assert_eq!(sum, Vec2::new(11f32, 22f32))
    }

    #[test]
    fn sub() {
        let dif = Vec2::new(10f32, 20f32) - Vec2::new(1f32, 2f32);
        assert_eq!(dif, Vec2::new(9f32, 18f32))
    }

    #[test]
    fn sub_assign() {
        let mut dif = Vec2::new(10f32, 20f32);
        dif -= Vec2::new(1f32, 2f32);
        assert_eq!(dif, Vec2::new(9f32, 18f32))
    }
}