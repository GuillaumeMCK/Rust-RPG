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
    /// The current difficulty - the enemies will speed up over time
    pub difficulty: f32,
    /// Information about the Message to draw on the screen
    pub message: Option<Message>,
    /// Score
    pub score: u32,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new<R: Rng>(ref mut rng: &mut R, size: Size) -> GameState {
        GameState {
            world: World::new(rng, size),
            difficulty: 0.0,
            message: Some(WELCOME_MESSAGE),
            score: 0,
        }
    }

    /// Called when the game is over - displays a message onscreen
    pub fn game_over(&mut self) {
        self.message = Some(GAMEOVER_MESSAGE);
    }

    /// Reset our game-state
    pub fn reset(&mut self, rng: &mut impl Rng) {
        // Create a new world
        self.world = World::new(rng, self.world.size);

        // Reset score
        self.score = 0;

        // Reset difficulty
        self.difficulty = 0.0;

        // Reset message
        self.message = None;

        // Remove all enemies and powerups
        self.world.enemies.clear();
        self.world.powerups.clear();
    }
}
