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

/// A trait that provides collision detection for objects with a position and a radius
///
/// For collision purposes, all objects are treated as circles
pub trait Collide: Position {
    /// Returns the radius of the object
    fn radius(&self) -> f32;

    /// Returns the diameter of the objects
    fn diameter(&self) -> f32 {
        self.radius() * 2.0
    }

    /// Returns true if the two objects collide and false otherwise
    fn collides_with<O: Collide>(&self, other: &O) -> bool {
        let radii = self.radius() + other.radius();
        self.position().squared_distance_to(other.position()) < radii * radii
    }
}
