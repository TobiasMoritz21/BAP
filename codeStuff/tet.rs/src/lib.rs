//! Crate Lib
pub mod tetromino;
pub mod field;

/// Helper struct for 2D-Coords
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
