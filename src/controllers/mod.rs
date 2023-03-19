//! This module contains the game logic
//!
//! There are three main controllers: collisions, input and time

pub use self::input::{
    Actions,
    InputController,
};

// mod collisions;
mod input;

#[derive(Debug)]
pub enum Event {
    GameStart,
    GameOver,
    GameWon,
    Attack,
    Defend,
    EnemyKilled,
    EnemySpawned,
}
