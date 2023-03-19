use std::path::Path;

use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Color, DrawParam, Image};
use ggez::graphics::spritebatch::SpriteDataBatch;
use ggez::mint::Point2;

impl Atlas {
    pub fn parse_atlas_json(texture_atlas_file: &Path) -> Self {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(texture_atlas_file).expect("Couldn't find the texture_atlas file");
        let buf_reader = BufReader::new(file);
        serde_json::from_reader(buf_reader).expect("Couldn't create texture atlas")
    }

    /// Returns a sprite from the Atlas.
    pub fn create_sprite(&self, sprite_name: &str) -> SpriteData {
        let width = self.meta.size.w as f32;
        let height = self.meta.size.h as f32;
        let atlas_rect = graphics::Rect::new(0.0, 0.0, width, height);

        if let Some(sprite_data) = self.frames.iter().find(|d| d.filename == sprite_name) {
            SpriteData::new(
                graphics::Rect::fraction(
                    sprite_data.frame.x as f32,
                    sprite_data.frame.y as f32,
                    sprite_data.frame.w as f32,
                    sprite_data.frame.h as f32,
                    &atlas_rect,
                ),
                sprite_data.frame.w as f32,
                sprite_data.frame.h as f32,
            )
        } else {
            unimplemented!("Not handling failure to find sprite");
        }
    }
}


pub struct Texture {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Texture {
    pub fn draw_params(&self, pos: Point2<f32>) -> graphics::DrawParam {
        DrawParam::new()
            .src(self.rect())
            .dest(pos)
    }

    fn rect(&self) -> graphics::Rect {
        graphics::Rect::fraction(self.x, self.y, self.width, self.height, &Default::default())
    }
}

struct GameState {
    spritebatch: SpriteDataBatch,
}

impl GameState {
    fn new(context: &mut Context) -> GameResult<GameState> {
        let path: &Path = Path::new("/atlas/landscape.png");
        let image = Image::new(context, path)?;
        let mut spritebatch = SpriteDataBatch::new(image);

        let mut texture_1 = Texture {
            id: 1,
            x: 0.0,
            y: 297.0,
            width: 133.0,
            height: 127.0,
        };

        /// Add the draw command to the sprite batch.
        let draw_param_1 = texture_1.draw_params(Point2 { x: 0.0, y: 0.0 });

        spritebatch.add(draw_param_1);

        Ok(GameState { spritebatch })
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        // update game state here
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, [0.1, 0.2, 0.3, 1.0].into()); // clear the screen to a dark blue


        let red_rect = graphics::Mesh::new_rectangle(
            context,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 297.0, 133.0, 127.0),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(context, &red_rect, DrawParam::new())?;


        let dp = self.spritebatch.get_sprites()[0];
        graphics::draw(context, &self.spritebatch, dp)?;


        graphics::present(context)?;
        Ok(())
    }
}

fn main() -> GameResult<()> {
    let (mut context, mut event_loop) = ggez::ContextBuilder::new("my_game", "me")
        .build()?;
    let game_state = GameState::new(&mut context)?;
    ggez::event::run(context, event_loop, game_state)
}
