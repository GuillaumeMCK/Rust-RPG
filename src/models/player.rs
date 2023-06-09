use rand::Rng;
use geometry::{Size, Vector, Position, Point, Collide};
use geometry_derive::Position;
use crate::models::{Powerup, PowerupKind};

/// The `Player` is the hero controlled by the user
#[derive(Default, Position)]
pub struct Player {
    pub is_dead: bool,
    pub vector: Vector,
    pub powerup: Option<PowerupKind>,
}

impl Player {
    /// Create a new `Player` at the given position
    pub fn new(position: Point) -> Player {
        Player {
            is_dead: true,
            vector: Vector::new(position, 1.0),
            powerup: None,
        }
    }
}



impl Collide for Player {
    fn radius(&self) -> f32 {
        6.0
    }
}
