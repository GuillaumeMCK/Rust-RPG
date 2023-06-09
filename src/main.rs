#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
//----------------------------------------------
// KISS - Keep It Simple,
// Stupid - Simplest Thing That Could Possibly Work
// DRY - Don't Repeat Yourself
// YAGNI - You Ain't Gonna Need It
//----------------------------------------------

use ggez::{Context, GameError, GameResult};
use ggez::event::{self, KeyCode, KeyMods};
use rand::prelude::ThreadRng;
use structopt::StructOpt;

use geometry::Size;

use crate::{
    controllers::{
        Event,
        InputController,
        CollisionsController
    },
    game_state::GameState,
    view::{
        init_rendering_ctx,
        Resources,
    },
};
use crate::controllers::TimeController;

mod view;
mod controllers;
mod game_state;
mod models;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rust RPG", about = "A simple RPG game written in Rust")]
struct Opt {
    /// The width of the game window
    #[structopt(short, long, default_value = "1400")]
    width: f32,
    /// The height of the game window
    #[structopt(short, long, default_value = "1000")]
    height: f32,
}

/// This struct contains the application's state
pub struct ApplicationState {
    // Keep track of window focus to play/pause the game
    has_focus: bool,
    // Resources holds our loaded font, images and sounds
    resources: &'static mut Resources,
    // The game state contains all information needed to run the game
    game_state: GameState,
    // Time controller keeps track of the time that has passed since the last frame
    // and the time that has passed since the last shot
    time_controller: TimeController,
    // The input controller keeps track of the actions that are triggered by the player
    input_controller: InputController,
    // The event buffer keeps track of events that trigger sounds, so we can separate
    // sound playing from the game logic
    event_buffer: Vec<Event>,
    // A source of randomness
    rng: ThreadRng,
}

impl ApplicationState {
    /// Simply creates a new application state
    fn new(ctx: &mut Context, game_size: Size) -> GameResult<ApplicationState> {
        let mut rng = rand::thread_rng();

        // Initialize the game resources
        Resources::init(ctx);

        // Return the application state in a game result
        Ok(ApplicationState {
            has_focus: true,
            resources: Resources::instance(),
            game_state: GameState::new(&mut rng, game_size),
            time_controller: TimeController::new(),
            input_controller: InputController::new(),
            event_buffer: vec![],
            rng,
        })
    }

    /// This will be called when the game needs to be reset
    fn reset(&mut self) {
        // reset the game state
        self.game_state.reset(&mut self.rng);
        // add a game start event to the event buffer
        self.event_buffer.push(Event::GameStart);
    }
}

/// Implement the ggez event handler trait for our application state to handle
/// user inputs
impl event::EventHandler for ApplicationState {
    /// This will be called every time the game loop updates so we can update the game state here
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Pause the game if the window has no focus
        if !self.has_focus {
            return Ok(());
        }

        // Update game state, and check for collisions
        let duration = ggez::timer::delta(ctx);
        self.time_controller.update_seconds(
            duration,
            self.input_controller.actions(),
            &mut self.game_state,
            &mut self.event_buffer,
            &mut self.rng,
        );

        CollisionsController::handle_collisions(&mut self.game_state, &mut self.time_controller, &mut self.event_buffer);

        Ok(())
    }

    // This is called when ggez wants us to draw our game
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        view::play_sounds(ctx, &mut self.event_buffer, self.resources)?;
        view::render_game(ctx, self)
    }
}

fn main() {
    // Parse the command line arguments
    let opt = Opt::from_args();

    // Create the game size
    let game_size = Size::new(opt.width, opt.height);

    // Define the resource directory
    let resource_dir = std::path::PathBuf::from("../resources");

    // Build the window
    let (mut ctx, event_loop) = init_rendering_ctx(game_size, resource_dir).unwrap();

    // Load the application state and start the event loop
    let mut state = ApplicationState::new(&mut ctx, game_size).unwrap();

    // Run the event loop
    event::run(ctx, event_loop, state);
}


// use crate::core::entities::{Entity, Player};
// use crate::core::items::{ItemInfos, WeaponType};
// use crate::core::utils::Position;

// pub mod core;
//
// fn main() {
//     let mut item = ItemInfos::new("Sword".to_string(), "A sword".to_string(), 1, 1);
//
//     let mut weapon = WeaponType::Sword {
//         attack: 1,
//         range: 1,
//         infos: item,
//     };
//
//     println!("{:?}", weapon);
//
//     let mut hero = Player::random("Hero", Position::new(0, 0));
//
//     println!("{}, {}, {}, {}, {}, {}", hero.get_name(), hero.get_health(), hero.get_attack(), hero.get_defense(), hero.get_strength(), hero.get_weight());
// }
