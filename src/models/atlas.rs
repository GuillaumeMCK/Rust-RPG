use ggez::{
    graphics::{
        DrawParam,
        Image,
        Rect,
        spritebatch::SpriteBatch,
    },
    mint::Point2,
};
use ggez::graphics::Color;
use ron::de::from_reader;
use serde::Deserialize;
use strum_macros::Display;

use geometry::Size;

use crate::view::SCALE;

#[derive(Debug, PartialEq, Clone, Display, Deserialize)]
pub enum Type {
    Landscape,
    TowersBrown,
    TowersGrey,
    TowersRed,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Frame {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasInfo {
    /// The name of the atlas.
    pub name: Type,
    /// Path to the atlas image.
    pub path: String,
    /// The size of the atlas.
    pub size: (i32, i32),
    /// The frames of the atlas.
    frames: Vec<Frame>,
}


#[derive(Debug)]
pub struct Atlas {
    pub image: Image,
    pub meta: AtlasInfo,
}

impl Atlas {
    /// Read the atlas image from the given path and create a sprite batch
    pub fn create_sprite_batch(&self) -> SpriteBatch {
        let mut sp = SpriteBatch::new(self.image.clone());
        sp.set_filter(ggez::graphics::FilterMode::Nearest);
        sp
    }

    /// Returns a sprite from the Atlas.
    pub fn create_sprite(&self, sprite_ref: SpriteRef) -> Option<Sprite> {
        if let Some(sprite) = self.get_frame(sprite_ref) {
            let atlas_rect = Rect::new(
                0.0,
                0.0,
                self.meta.size.0 as f32,
                self.meta.size.1 as f32,
            );
            Some(
                Sprite::new(
                    Rect::fraction(
                        sprite.x as f32,
                        sprite.y as f32,
                        sprite.w as f32,
                        sprite.h as f32,
                        &atlas_rect,
                    ),
                    sprite.w as f32,
                    sprite.h as f32,
                )
            )
        } else {
            unimplemented!("Frame not found");
        }
    }

    /// Returns the frame for the given `SpriteRef`.
    fn get_frame(&self, value: SpriteRef) -> Option<&Frame> {
        match value {
            SpriteRef::Name(name) => self.meta.frames.iter().find(|f| f.name == name),
            SpriteRef::Index(index) => self.meta.frames.get(index),
        }
    }
}


#[derive(Debug)]
pub struct AtlasData {
    atlas: Vec<Atlas>,
}

impl AtlasData {
    /// Creates a new `AtlasData` from the given RON file.
    pub fn parse_file(ctx: &mut ggez::Context, ron_path: &str) -> Self {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(ron_path).expect("Something went wrong reading the file");
        let buf_reader = BufReader::new(file);
        let atlas_infos: Vec<AtlasInfo> = from_reader(buf_reader).expect("Failed to parse RON");

        // Fetch the image for the given `RawAtlas`.
        let atlas: Vec<Atlas> = atlas_infos
            .iter()
            .map(|a| {
                let image = Image::new(ctx, &a.path).expect("Atlas image not found");
                Atlas { image, meta: a.clone() }
            })
            .collect();
        AtlasData { atlas }
    }

    /// Returns the `Atlas` for the given `Type`.
    pub fn get(&self, atlas_type: Type) -> Option<&Atlas> {
        self.atlas.iter().find(|a| a.meta.name == atlas_type)
    }
}

/// A reference to a sprite in an atlas.
/// Use `SpriteRef::Name` to reference a sprite by name.
/// Use `SpriteRef::Index` to reference a sprite by index.
pub enum SpriteRef<'a> {
    Name(&'a str),
    Index(usize),
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub rect: Rect,
    pub scale: Point2<f32>,
    pub width: f32,
    pub height: f32,
}

impl Sprite {
    /// Creates a new `Sprite`.
    pub fn new(rect: Rect, width: f32, height: f32) -> Self {
        Sprite { rect, width, height, scale: Point2 { x: SCALE, y: SCALE } }
    }

    /// Get draw params for the `Sprite` at the given position.
    pub fn draw_params(&self, pos: [f32; 2]) -> DrawParam {
        DrawParam::new()
            .src(self.rect)
            .scale(self.scale)
            .dest(pos)
    }

    /// Returns the bounding box for this sprite.
    pub fn get_bound_box(&self) -> Rect {
        let mut rect = self.rect.clone();
        rect.scale(self.scale.x, self.scale.y);
        rect
    }
}
