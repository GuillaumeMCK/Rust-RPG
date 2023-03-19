//! Traits used by the models
use super::{Point};

/// A trait for objects that occupy a position in space
pub trait Position {
    /// Returns the x coordinate of the object
    fn x(&self) -> f32;

    /// Returns a mutable reference to the x coordinate
    fn x_mut(&mut self) -> &mut f32;

    /// Returns the y coordinate of the object
    fn y(&self) -> f32;

    /// Returns a mutable reference to the y coordinate
    fn y_mut(&mut self) -> &mut f32;

    /// Returns the position of the object
    fn position(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}
