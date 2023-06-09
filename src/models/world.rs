use ggez::mint::Point2;
use rand::Rng;

use geometry::{Point, Size};

use crate::{
    models::{
        atlas::AtlasData,
        Enemy,
        Map,
        Player,
    },
    view::{
        SCALE,
        TILE_SIZE,
    },
};
use crate::models::Powerup;

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub powerups: Vec<Powerup>,
    pub map: Map,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        let map_size = (size.height / TILE_SIZE.height) as usize;
        let map = Map::new(rng, map_size);
        println!("map_size: {}", map_size);
        World {
            player: Player::new(map.get_random_point(rng)),
            enemies: vec![],
            powerups: vec![],
            map,
            size,
        }
    }

    /// Returns the map position
    pub fn map_position(&self) -> Point {
        // Calculate the center of the screen
        let mut x = self.size.width / 2.0;
        let mut y = self.size.height / 2.0;

        // Remove the first tile size to center the map
        // x -= TILE_SIZE.scale(SCALE).width / 2.0;
        // y -= TILE_SIZE.scale(SCALE).height / 2.0;

        // Return the position
        Point::new(x, y)
    }
}
