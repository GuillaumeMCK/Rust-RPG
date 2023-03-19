use ggez::mint::Point2;
use rand::Rng;

use geometry::Size;

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

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub map: Map,
    pub size: Size,
}

impl World {
    /// Returns a new world of the given size
    pub fn new<R: Rng>(rng: &mut R, size: Size) -> World {
        let map_size = (size.height / TILE_SIZE.height) as usize;
        println!("map_size: {}", map_size);
        World {
            player: Player::random(rng, Size::new(map_size as f32, map_size as f32)),
            enemies: vec![],
            map: Map::new(rng, map_size),
            size,
        }
    }

    /// Returns the map position
    pub fn map_position(&self) -> Point2<f32> {
        // Calculate the center of the screen
        let mut x = self.size.width / 2.0;
        let mut y = self.size.height / 2.0;

        // Remove the first tile size to center the map
        // x -= TILE_SIZE.scale(SCALE).width / 2.0;
        // y -= TILE_SIZE.scale(SCALE).height / 2.0;

        Point2 { x, y }
    }
}
