//! Playing Field
//!
//! (0.0, 0.0)  ...     (1.0, 0.0)
//! ...         ...     ...
//! (0.0, 1.0)  ...     (1.0, 1.0)
//!
use crate::Vector2;

/// Base Field Struct
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub x_dims: f32,
    pub y_dims: f32,
    pub occupied: Vec<Vector2>
}
