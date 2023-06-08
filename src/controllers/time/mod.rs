mod timeout;
mod timeout_queue;
mod timer;

use std::{mem, f32};
use std::time::Duration;
use rand::Rng;
use geometry::{Point, Vector};

use crate::{
    controllers::{
        Event,
        input::Actions,
    },
    game_state::GameState,
};
use crate::models::Enemy;

use self::timer::Timer;
pub use self::timeout::Timeout;
use self::timeout_queue::TimeoutQueue;

// Constants related to time
const ATTACKS_PER_SECOND: f32 = 30.0;
const ATTACKS_RATE: f32 = 1.0 / ATTACKS_PER_SECOND;

const ENEMY_SPAWNS_PER_SECOND: f32 = 5.0;
const ENEMY_SPAWN_RATE: f32 = 1.0 / ENEMY_SPAWNS_PER_SECOND;

// Constants related to movement
// Speed is measured in pixels per second
// Rotation speed is measured in radians per second
const ADVANCE_SPEED: f32 = 200.0;
const BULLET_SPEED: f32 = 500.0;
const ENEMY_SPEED: f32 = 100.0;
const ROTATE_SPEED: f32 = 2.0 * f32::consts::PI;
const STAR_BASE_SPEED: f32 = 50.0;

pub const PLAYER_GRACE_AREA: f32 = 200.0;

pub struct TimeController {
    /// The duration of the current game, since the last restart
    current_time: Duration,
    /// A timer to trigger creation of bullets
    shoot_timer: Timer,
    /// A timer to spawn enemies
    enemy_timer: Timer,
    /// Scheduled events that should happen in the future
    scheduled_timeouts: TimeoutQueue,
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController {
            current_time: Duration::from_secs(0),
            shoot_timer: Timer::from_seconds(ATTACKS_RATE),
            enemy_timer: Timer::from_seconds(ENEMY_SPAWN_RATE),
            scheduled_timeouts: TimeoutQueue::new(),
        }
    }

    // Called when the game is reset
    pub fn reset(&mut self) {
        let _ = mem::replace(self, TimeController::new());
    }

    pub fn schedule_timeout(&mut self, offset: Duration, timeout: Timeout) {
        self.scheduled_timeouts.push(self.current_time + offset, timeout);
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds<R: Rng>(
        &mut self,
        dt: Duration,
        actions: &Actions,
        state: &mut GameState,
        events: &mut Vec<Event>,
        rng: &mut R
    ) {
        self.current_time += dt;

        let dt = util::duration_to_seconds(dt);
        state.difficulty += dt / 100.0;

        // Check if we have any events that are scheduled to run, and if so, run them now
        if let Some(when) = self.scheduled_timeouts.peek() {
            if when <= self.current_time {
                self.scheduled_timeouts.pop().unwrap().handle(state);
            }
        }


        // Only modify player/powerups if player is alive
        if !state.world.player.is_dead {
            self.update_player(dt, actions, state);
            self.update_powerups(dt, state, rng);
        }

        self.update_enemies(dt, state, events, time_slow, rng);
        self.update_stars(dt, state, time_slow);
    }

    // Updates the position and rotation of the player
    fn update_player(&mut self, dt: f32, actions: &Actions, state: &mut GameState) {
        if !state.world.player.is_dead {
            if actions.rotate_left {
                *state.world.player.direction_mut() += -ROTATE_SPEED * dt;
            } else if actions.rotate_right {
                *state.world.player.direction_mut() += ROTATE_SPEED * dt;
            }

            // Set speed and advance the player with wrap around
            let speed = if actions.boost {
                2.0 * ADVANCE_SPEED
            } else {
                ADVANCE_SPEED
            };
            state
                .world
                .player
                .advance_wrapping(dt * speed, state.world.size);

            // Cool down the player's gun
            state.world.player.gun.cool_down(dt);
        }
    }

    // Updates positions of enemies, and spawns new ones when necessary
    fn update_enemies<R: Rng>(
        &mut self,
        dt: f32,
        state: &mut GameState,
        events: &mut Vec<Event>,
        rng: &mut R
    ) {
        // Spawn enemies at random locations
        self.enemy_timer.update(self.current_time, || {
            let player_pos: &Vector = &state.world.player.vector;
            let mut enemy_pos;
            // We loop here, just in case the new enemy random position is exactly equal
            // to the players current position, this would break our calculations below
            loop {
                enemy_pos = Vector::random(rng, state.world.size);
                if enemy_pos.position != player_pos.position {
                    break;
                }
            }

            // Check if the newly spawned enemy is inside the player's grace area,
            // if so, we push its spawn point to the edge of the area
            if enemy_pos
                .position
                .intersect_circle(&player_pos.position, PLAYER_GRACE_AREA)
            {
                // Treat the player as the centre of a circle with radius PLAYER_GRACE_AREA
                let Point { x: cx, y: cy } = player_pos.position;
                let dp: Point = enemy_pos.position - player_pos.position;
                // Calculate the angle between the player's position and the enemy's
                let angle = (dp.y).atan2(dp.x);
                // Use that to place the enemy on the edge of the circle surrounding the player
                enemy_pos.position = Point {
                    x: cx + PLAYER_GRACE_AREA * angle.cos(),
                    y: cy + PLAYER_GRACE_AREA * angle.sin(),
                };
            }

            let new_enemy = Enemy::new(enemy_pos);
            state.world.enemies.push(new_enemy);
            events.push(Event::EnemySpawned);
        });

        // Move enemies in the player's direction if player is alive, otherwise let them drift in
        // the direction they're facing
        for enemy in &mut state.world.enemies {
            if !state.world.player.is_dead {
                let base_speed = if time_slow {
                    ENEMY_SPEED - 75.0
                } else {
                    ENEMY_SPEED
                };
                enemy.update(
                    dt * base_speed + state.difficulty,
                    state.world.player.vector.position,
                    state.world.size,
                );
            } else {
                enemy.advance(dt * ENEMY_SPEED);
            }
        }
    }
}
