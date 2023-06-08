use ggez::graphics::{DrawParam, FilterMode};
use ggez::graphics::spritebatch::SpriteBatch;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use geometry::{Point, Size, Vector};

use crate::{
    models::atlas::{
        self,
        AtlasData,
        Sprite,
        SpriteRef,
    },
    view::{
        Resources,
        SCALE,
        TILE_SIZE,
    },
};

#[derive(Debug)]
pub struct Map {
    /// The size of the map
    pub size: usize,
    /// The layers of the map
    layers: Vec<Layer>,
    // 0: top, 1: right, 2: bottom, 3: left
    pub playable_area: [Point; 4],
}

impl Map {
    pub fn new<R: Rng>(rng: &mut R, size: usize) -> Map {
        let mut layers = Self::generate_map(rng, size);

        let mut playable_area = Self::calculate_playable_area(&layers[0]);

        println!("playable_area: {:?}", playable_area);

        Map {
            size,
            layers,
            playable_area,
        }
    }

    /// Creates a new map with the given size
    fn generate_map<R: Rng>(rng: &mut R, size: usize) -> Vec<Layer> {
        // Selecte the landscape atlas
        let landscape = Resources::instance().atlas_data.get(atlas::Type::Landscape).unwrap();
        // Create a sprite batch for the landscape
        let mut sprite_batch = landscape.create_sprite_batch();

        // Create a vector to hold the layers
        let mut layers = vec![];

        // Generate a random seed
        let seed = rng.gen::<u32>();

        // Create a perlin noise generator
        let perlin = Perlin::new(seed);
        let mut gen_map: Vec<Vec<u32>> = vec![vec![0; size]; size];

        gen_map.iter_mut().enumerate().for_each(|(x, row)| {
            row.iter_mut().enumerate().for_each(|(y, val)| {
                *val = (perlin.get([x as f64 / 25.0, y as f64 / 25.0]) * 25.0) as u32;
            });
        });

        println!("{:?}", gen_map);

        let mut layer: Vec<Vec<Tile>> = Vec::new();

        let half_tile_width = TILE_SIZE.width / 2.0;
        let half_tile_height = TILE_SIZE.height / 2.0;

        let map_height = half_tile_height * (gen_map.len() as f32);

        for (x_idx, row_of_gen_map) in gen_map.iter().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (y_idx, value) in row_of_gen_map.iter().enumerate() {
                if let Some(sprite) = landscape.create_sprite(SpriteRef::Index(*value as usize)) {
                    let tile = Tile {
                        position: Self::calculate_tile_position(x_idx, y_idx, &sprite, map_height, half_tile_width),
                        sprite,
                    };
                    sprite_batch.add(tile.sprite.draw_params(tile.position.point2()));
                    row.push(tile);
                } else {
                    println!("No sprite for {}", value);
                }
            }
            layer.push(row);
        }

        layers.push(Layer {
            level: 0,
            data: layer,
            sprite_batch,
        });

        layers
    }

    /// Calculate the isometric position of the tile
    fn calculate_tile_position(x_idx: usize, y_idx: usize, sprite: &Sprite, map_height: f32, half_tile_width: f32) -> Point {
        let half_tile_width = TILE_SIZE.width / 2.0;
        let half_tile_height = TILE_SIZE.height / 2.0;

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

        Point::new(x, y)
    }

    /// Calculate the playable area of the map
    fn calculate_playable_area(base_layer: &Layer) -> [Point; 4] {
        let rows_nb = base_layer.data.len();
        let cols_nb = base_layer.data[0].len();
        [
            // with the first row of tiles we can get the top and left points
            base_layer.data[0][0].position, // top,
            base_layer.data[0][cols_nb - 1].position, // right,
            // with the last row of tiles we can get the bottom and right points
            base_layer.data[rows_nb - 1][cols_nb - 1].position, // bottom,
            base_layer.data[rows_nb - 1][0].position, // left,
        ]
    }

    /// Returns the reference to the layers
    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    /// Returns the number of layers
    pub fn layers_len(&self) -> usize {
        self.layers.len()
    }

    pub fn get_random_point<R: Rng>(&self, rng: &mut R) -> Point {
        self.layers[0].data[rng.gen_range(0..self.size)][rng.gen_range(0..self.size)].position
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
