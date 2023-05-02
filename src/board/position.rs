use std::ops;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl ops::Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Add<Vector2D> for Position {
    type Output = Self;

    fn add(self, other: Vector2D) -> Self {
        Self {x: (self.x as i64 + other.x) as usize, y: (self.y as i64 + other.y) as usize}
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}

impl Vector2D {
    pub fn _rotate_right(& mut self) {
        let z = self.x;
        self.x = - self.y;
        self.y = z;
    }

    pub fn _rotate_left(& mut self) {
        let z = self.y;
        self.y = - self.x;
        self.x = z;
    }
}

impl ops::Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}
