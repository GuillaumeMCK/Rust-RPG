use ggez::graphics::DrawParam;
use ggez::graphics::spritebatch::SpriteBatch;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use geometry::{Point, Size, Vector};

use crate::{
    models::{
        atlas::{
            self,
            AtlasData,
            Sprite,
            SpriteRef,
        },
    },
    view::{
        SCALE,
        TILE_SIZE,
        Resources
    },
};

#[derive(Debug)]
pub struct Map {
    pub size: usize,
    layers: Vec<Layer>,
}

impl Map {
    pub fn new<R: Rng>(rng: &mut R, size: usize) -> Map {
        let landscape = Resources::instance().atlas_data.get(atlas::Type::Landscape).unwrap();
        let mut sprite_batch = landscape.create_sprite_batch();

        let mut layers = vec![];

        let gen_map: Vec<Vec<u32>> = Self::generate_map(rng, size);
        let mut layer: Vec<Vec<Tile>> = Vec::new();

        let half_tile_width = TILE_SIZE.width / 2.0;
        let half_tile_height = TILE_SIZE.height / 2.0;

        let map_height = half_tile_height * (gen_map.len() as f32);

        for (x_idx, row_of_gen_map) in gen_map.iter().enumerate() {
            let mut new_tiles_row = Vec::new();

            for (y_idx, value) in row_of_gen_map.iter().enumerate() {
                if let Some(sprite) = landscape.create_sprite(SpriteRef::Index(*value as usize)) {
                    // Calculate the isometric position of the tile
                    let mut x = (x_idx as f32) * half_tile_width - (y_idx as f32) * half_tile_width;
                    let mut y = (y_idx as f32) * half_tile_height + (x_idx as f32) * half_tile_height;

                    // Subtracting the difference between the tile size and the sprite size
                    // to center the sprite in the tile
                    x -= sprite.width - TILE_SIZE.width;
                    y -= sprite.height - TILE_SIZE.height;

                    // Add offset to center the map
                    x -= half_tile_width;
                    y -= map_height - half_tile_height;

                    // Scale the tile
                    x *= SCALE;
                    y *= SCALE;

                    // Create the tile
                    let tile = Tile {
                        position: Point::new(x, y),
                        sprite,
                    };
                    // Add the tile to the sprite batch and the layer
                    sprite_batch.add(tile.sprite.draw_params(tile.position.point2()));
                    new_tiles_row.push(tile);
                } else {
                    println!("No sprite for {}", value);
                }
            }
        }

        layers.push(Layer {
            level: 0,
            data: layer,
            sprite_batch,
        });

        Map {
            size,
            layers,
        }
    }

    fn generate_map<R: Rng>(rng: &mut R, size: usize) -> Vec<Vec<u32>> {
        let seed = rng.gen::<u32>();

        let perlin = Perlin::new(seed);
        let mut map: Vec<Vec<u32>> = vec![vec![0; size]; size];

        map.iter_mut().enumerate().for_each(|(x, row)| {
            row.iter_mut().enumerate().for_each(|(y, val)| {
                let value = perlin.get([(x as f64) / 50.0, (y as f64) / 50.0]) * 50.0;

                // cap with mod 25
                let value = value % 25.0;

                *val = value as u32;
            });
        });

        println!("{:?}", map);
        map
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn layers_len(&self) -> usize {
        self.layers.len()
    }
}


#[derive(Debug)]
pub struct Tile {
    pub position: Point,
    pub sprite: Sprite,
}

impl Tile {
    pub fn draw(&self, batch: &mut SpriteBatch) {
        batch.add(self.sprite.draw_params(self.position.point2()));
    }
}

#[derive(Debug)]
pub struct Layer {
    pub level: i32,
    pub data: Vec<Vec<Tile>>,
    pub sprite_batch: SpriteBatch,
}
