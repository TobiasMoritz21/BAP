//! Tetromino

/// Base Tetromino Struct
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tetromino {
    pub pos: Vector2,
    pub orientation: Rot,
    pub shape: Shape,
}

impl Tetromino {
    pub fn new(pos: Vector2, shape: Shape) -> Self {
        Self {
            pos,
            orientation: Rot::0,
            shape,
        }
    }

    pub fn update(&mut self, rotation: Rot) {
        self.pos.y += 1.0;
        self.orientation = rotation;
    }
}

/// Rotation Enum
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rot {
    0,
    90,
    180,
    270,
}

/// Tetromino Type Enum
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    O,
    T,
    I,
    L,
    J,
    S,
    Z,
}
