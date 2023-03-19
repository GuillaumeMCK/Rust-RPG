use rand::Rng;
use rand::rngs::ThreadRng;

use geometry::Size;

use crate::models::World;

/// This is a message that will be drawn to the screen. When it's shown on the screen the game
/// will be waiting for user input
pub struct Message {
    pub title: &'static str,
    pub subtitle: &'static str,
}

/// The Message to show when the game starts
const WELCOME_MESSAGE: Message = Message {
    title: "Welcome to Rust-RPG",
    subtitle: "Press any key to start",
};

/// The Message to show when the game is over
const GAMEOVER_MESSAGE: Message = Message {
    title: "Game Over",
    subtitle: "Press any key to restart",
};

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// Information about the Message to draw on the screen
    pub message: Option<Message>,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new<R: Rng>(ref mut rng: &mut R, size: Size) -> GameState {
        GameState {
            world: World::new(rng, size),
            message: Some(WELCOME_MESSAGE),
        }
    }

    /// Called when the game is over - displays a message onscreen
    pub fn game_over(&mut self) {
        self.message = Some(GAMEOVER_MESSAGE);
    }

    /// Reset our game-state
    pub fn reset(&mut self, rng: &mut impl Rng) {
        self.world = World::new(rng, self.world.size);
        self.message = Some(WELCOME_MESSAGE);
    }
}
