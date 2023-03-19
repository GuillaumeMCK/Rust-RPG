use rand::Rng;
use geometry::{Size, Vector, Position};
use geometry_derive::Position;
use ggez::mint::Point2;

/// The `Player` is the hero controlled by the user
#[derive(Default, Position)]
pub struct Player {
    pub is_dead: bool,
    vector: Vector,
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Player {
        Player {
            is_dead: true,
            vector: Vector::random(rng, bounds),
        }
    }

    /// Radius of the player
    pub fn radius(&self) -> f32 {
        6.0
    }
}


//
// impl Collide for Player {
//     fn radius(&self) -> f32 {
//         6.0
//     }
// }
