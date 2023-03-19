use ggez::{Context, GameResult};
use ggez::audio::SoundSource;

use crate::{
    controllers::Event,
    view::Resources,
};

use self::Event::*;

pub fn play_sounds(ctx: &Context, events: &mut Vec<Event>, resources: &mut Resources) -> GameResult<()> {
    for event in events.drain(..) {
        match event {
            GameStart => resources.jukebox.music.play_detached(ctx)?,
            GameOver => resources.jukebox.lose.play_detached(ctx)?,
            GameWon => resources.jukebox.win.play_detached(ctx)?,
            Attack => resources.jukebox.hit.play_detached(ctx)?,
            Defend => resources.jukebox.defend.play_detached(ctx)?,
            EnemyKilled => (),
            EnemySpawned => (),
        }
    }
    Ok(())
}
