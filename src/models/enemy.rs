use rand::Rng;
use geometry::{Size, Vector, Position, Point, Collide};
use geometry_derive::Position;
use crate::models::{Powerup, PowerupKind};

/// The `Enemy` is the hero controlled by the user
#[derive(Default, Position)]
pub struct Enemy {
    pub vector: Vector,
}

impl Enemy {
    /// Create a new `Enemy` at the given position
    pub fn new(position: Point) -> Enemy {
        Enemy {
            vector: Vector::new(position, 1.0),
        }
    }
}



impl Collide for Enemy {
    fn radius(&self) -> f32 {
        6.0
    }
}
