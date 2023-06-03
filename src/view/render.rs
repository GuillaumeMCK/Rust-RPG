use std;
use std::path::PathBuf;

use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::event::EventLoop;
use ggez::graphics::{self, Color, Drawable, DrawMode, DrawParam, FillOptions, Mesh, Rect, size, StrokeOptions, TextFragment};
use ggez::mint::Point2;

use geometry::{Point, Position, Size};

use crate::{
    ApplicationState,
    game_state::Message,
    models::{Layer, Map, Player, Tile, World},
    view::{
        colors,
        Resources,
    },
};

pub const TILE_SIZE: Size = Size { width: 128.0, height: 64.0 };
pub const SCALE: f32 = 0.6;

pub fn init_rendering_ctx(game_size: Size, resource_dir: PathBuf) -> GameResult<(Context, EventLoop<()>)> {
    let cb = ContextBuilder::new("Rust RPG", "GuillaumeMCK")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Rust RPG"))
        .window_mode(conf::WindowMode::default()
            .dimensions(game_size.width as f32, game_size.height as f32)
            .resizable(true));
    let ctx = cb.build()?;
    Ok(ctx)
}

/// Renders the game to the screen
pub fn render_game(ctx: &mut Context, app: &mut ApplicationState) -> GameResult<()> {
    // Clear everything
    graphics::clear(ctx, colors::BACKGROUND);

    // Render the world
    render_world(ctx, &app.game_state.world, app.resources)?;

    // Render a message if there is one set
    render_message(ctx, app)?;

    // Render the score
    let fragment = TextFragment::new(format!("Score: {}", 00)).scale(graphics::PxScale::from(24.0));
    let text = graphics::Text::new(fragment);
    let pt = Point::new(8.0, 8.0);
    graphics::draw(ctx, &text, DrawParam::new().dest(pt.point2()).color(colors::WHITE))?;

    // println!("{}", ggez::timer::fps(ctx));

    graphics::present(ctx)?; // NOTE: without this, the screen is not updated
    Ok(())
}

/// Renders the Message struct contained in the game's state to the middle of the screen
fn render_message(ctx: &mut Context, app: &mut ApplicationState) -> GameResult<()> {
    if let Some(ref message) = app.game_state.message {
        let Message { title, subtitle } = *message;
        let Size { width, height } = app.game_state.world.size;

        let w = width / 2.0;
        let h = height / 2.0;

        let mut draw_text = |text: &str, color: Color, font_size: f32, is_title: bool| {
            let fragment = TextFragment::new(text)
                .scale(graphics::PxScale::from(font_size))
                .color(color);
            let drawable = graphics::Text::new(fragment);

            let x = w - drawable.width(ctx) as f32 / 2.0;
            let y = if is_title { h - drawable.height(ctx) as f32 } else { h };

            graphics::draw(ctx, &drawable, DrawParam::new()
                .dest(Point::new(x, y).point2()))
        };

        draw_text(title, colors::GREY, 20.0, true)?;
        draw_text(subtitle, colors::WHITE, 14.0, false)?;
    }

    Ok(())
}

/// Renders the world and everything in it
pub fn render_world(ctx: &mut Context, world: &World, resources: &Resources) -> GameResult<()> {
    render_map(ctx, &world.map, resources, world.map_position())?;
    // render_enemy(ctx, world, resources)?;
    render_player(ctx, &world.player, resources)?;

    render_grid(ctx, world)?;
    // Finally draw the player as red
    // if !world.player.is_dead {
    //     render_player(ctx, &world.player, resources)?;
    // }

    Ok(())
}

/// Renders the map
pub fn render_map(ctx: &mut Context, map: &Map, resources: &Resources, pos: Point2<f32>) -> GameResult<()> {
    let layers: &Vec<Layer> = &map.layers();

    for layer in layers {
        layer.sprite_batch.draw(ctx, DrawParam::new().dest(pos))?;
    }

    Ok(())
}

/// Renders the player
pub fn render_player(ctx: &mut Context, player: &Player, resources: &Resources) -> GameResult<()> {
    let image = &resources.images.hero;
    let scale = player.radius() * 2.0 / resources.images.hero.width() as f32;
    graphics::draw(
        ctx,
        image,
        DrawParam::new()
            .dest(player.position().point2())
            .scale(Point::new(1.0, 1.0).point2()),
    )
}

/// Debug grid to show to middle of the screen
pub fn render_grid(ctx: &mut Context, world: &World) -> GameResult<()> {
    let Size { width, height } = world.size;
    let w = width / 2.0;
    let h = height / 2.0;

    let mut mesh = Mesh::new_line(
        ctx,
        &[Point::new(0.0, h).point2(), Point::new(width, h).point2()],
        1.0,
        colors::GREEN,
    )?;
    mesh.draw(ctx, DrawParam::new())?;

    mesh = Mesh::new_line(
        ctx,
        &[Point::new(w, 0.0).point2(), Point::new(w, height).point2()],
        1.0,
        colors::RED,
    )?;
    mesh.draw(ctx, DrawParam::new())?;

    Ok(())
}
