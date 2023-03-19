mod resources;
mod sound;
mod render;
mod colors;

pub use self::render::{init_rendering_ctx, render_game, TILE_SIZE, SCALE};
pub use self::resources::Resources;
pub use self::sound::play_sounds;
