use ggez::event::{KeyCode, KeyMods};

#[derive(Default)]
pub struct InputController {
    actions: Actions,
}

/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub attack: bool,
    pub defend: bool,
}

impl InputController {
    /// Create a new `InputController`
    pub fn new() -> InputController {
        InputController::default()
    }

    /// Returns a shared reference to the underlying actions
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    /// Processes a key press
    pub fn key_press(&mut self, keycode: KeyCode, _keymod: KeyMods) {
        self.handle_key(keycode, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, keycode: KeyCode, _keymod: KeyMods) {
        self.handle_key(keycode, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, keycode: KeyCode, pressed: bool) {
        match keycode {
            KeyCode::Up => self.actions.up = pressed,
            KeyCode::Down => self.actions.down = pressed,
            KeyCode::Left => self.actions.left = pressed,
            KeyCode::Right => self.actions.right = pressed,
            KeyCode::X => self.actions.attack = pressed,
            KeyCode::C => self.actions.defend = pressed,
            _ => (),
        }
    }
}
