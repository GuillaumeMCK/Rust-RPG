//! This module contains the game logic
//!
//! There are three main controllers: collisions, input and time

pub use self::input::{
    Actions,
    InputController,
};
pub use self::time::{PLAYER_GRACE_AREA, TimeController};
pub use self::collisions::CollisionsController;

mod collisions;
mod input;
mod time;

#[derive(Debug)]
pub enum Event {
    GameStart,
    GameOver,
    GameWon,
    Attack,
    Defend,
    EnemyKilled,
    EnemySpawned,
    PowerupGained
}
