use std::time::Duration;
use geometry::{Position, Collide};
use crate::{
    controllers::{
        Event,
        PLAYER_GRACE_AREA,
        time::{TimeController, Timeout},
    },
    game_state::GameState,
    models::{Enemy, PowerupKind},
};

const SCORE_PER_ENEMY: u32 = 10;
const POWERUP_DURATION: u64 = 10;

pub struct CollisionsController;

impl CollisionsController {
    pub fn handle_collisions(state: &mut GameState, time_controller: &mut TimeController, events: &mut Vec<Event>) {
        let old_enemy_count = state.world.enemies.len();

        let got_powerup =
            CollisionsController::handle_powerup_collisions(state, events);
        if got_powerup {
            // Powerups run out after `POWERUP_DURATION` seconds
            let offset = Duration::from_secs(POWERUP_DURATION);
            time_controller
                .schedule_timeout(offset, Timeout::RemovePowerup);
        }

        // If the player died then we set a timeout after which a game over message
        // will appear, and the user will be able to restart.
        let player_died =
            CollisionsController::handle_player_collisions(state, events);
        if player_died {
            let offset = Duration::from_secs(2);
            time_controller
                .schedule_timeout(offset, Timeout::ShowGameOverScreen);
        }

        let killed_enemies = (old_enemy_count - state.world.enemies.len()) as u32;
        state.score += SCORE_PER_ENEMY * killed_enemies;
    }

    /// Handles collisions between the player and powerups
    fn handle_powerup_collisions(state: &mut GameState, events: &mut Vec<Event>) -> bool {
        let mut gained_powerup = false;
        let player = &mut state.world.player;
        let powerups = &mut state.world.powerups;

        if !player.is_dead {
            if let Some((index, kind)) = powerups
                .iter()
                .enumerate()
                .find(|&(_, powerup)| powerup.collides_with(player))
                .map(|(index, powerup)| (index, powerup.kind))
            {
                gained_powerup = true;

                // Set player's powerup kind to the powerup we just picked up
                player.powerup = Some(kind);
                powerups.remove(index);

                events.push(Event::PowerupGained);
            }
        }

        return gained_powerup;
    }

    /// Handles collisions between the player and the enemies
    /// This function will return true if the player died
    fn handle_player_collisions(state: &mut GameState, events: &mut Vec<Event>) -> bool {
        let mut player_died = false;
        let player = &mut state.world.player;

        if !player.is_dead
            && state
                .world
                .enemies
                .iter()
                .any(|enemy| player.collides_with(enemy))
        {
            // Remove shield powerup from player, also killing any enemies within close range
            if let Some(PowerupKind::Shield) = player.powerup {
                player.powerup = None;

                let enemies = &mut state.world.enemies;
                // let particles = &mut state.world.particles;
                // CollisionsController::remove_surrounding_enemies(
                //     enemies,
                //     particles,
                //     player.position(),
                // );
                events.push(Event::EnemyKilled);
            } else {
                // Make an explosion where the player was killed
                let ppos = player.position();
                // Mark the player as dead (to stop drawing it on screen)
                player_died = true;
                player.is_dead = true;
                events.push(Event::GameOver);
            }
        }

        return player_died;
    }
}
