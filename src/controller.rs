//! An abstract controller for specifying actions.
//!
//! This controller acts like a bridge between any physical input and the
//! input which the internal gamestate can understand.
//!
//! When a key is pressed, this can be translated to a call to the controllers
//! `activate` function, and when the key is removed, this corresponds to a
//! call to the `deactivate` function.

use std::mem;

/// 'Controller Time' array.
///
/// This is defined to enforce type restrictions on external users of these
/// arrays, e.g. `History`.
pub type CTarray = [u64; 8];

/// 'Controller Active' array
pub type CAarray = [bool; 8];

/// Actions which are understood by the controller.
#[repr(usize)]
#[derive(Clone, Copy, Debug, Hash)]
#[allow(missing_docs)]
// When adding a new Action you MUST also alter the `History` module to
// match the new array size!
pub enum Action {
    MoveLeft, MoveRight, MoveDown, HardDrop,
    RotateLeft, RotateRight, Hold, Quit
}

impl From<usize> for Action {
	fn from(t: usize) -> Self {
		assert!(t < 8);
		unsafe { mem::transmute(t) }
	}
}

/// A controller stores the internal state as a series of known actions.
///
/// The active status of each action is stored, along with how long each action
/// has been active for. At its simplest, this/ controller parallels the
/// keystate of a keyboard.
#[derive(Default)]
pub struct Controller {
    /// The length each action has occured for in ticks.
    pub time: CTarray,

    /// Which actions are currently active.
    pub active: CAarray
}

impl Controller {
    /// Return a new controller instance.
    ///
    /// The controller specified will start with all actions in the inactive
    /// state, and time values zeroed.
    pub fn new() -> Controller {
        Controller { ..Default::default() }
    }

    /// Query if an action is currently active.
    pub fn active(&self, action: Action) -> bool {
        self.active[action as usize]
    }

    /// Query how long an action has been active for.
    pub fn time(&self, action: Action) -> u64 {
        self.time[action as usize]
    }

    /// Activate the specified action.
    ///
    /// The action will be set to active.
    /// Activating an already active timer has no effect.
    pub fn activate(&mut self, action: Action) {
        self.active[action as usize] = true;
    }

    /// Deactivate the specified action.
    ///
    /// The action will be set to inactive.
    /// Deactivating an already inactive action has no effect.
    pub fn deactivate(&mut self, action: Action) {
        self.active[action as usize] = false;
    }

    /// Deactivate all actions.
    ///
    /// This is useful when calculating actions based on the state of
    /// an object, rather than explicitly via events.
    ///
    /// This does not reset the internal time of each action to 0.
    pub fn deactivate_all(&mut self) {
        for i in 0..self.active.len() {
            self.active[i] = false;
        }
    }

    /// Update all active actions and increment their timers.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::controller::{Action, Controller};
    ///
    /// let mut controller = Controller::new();
    /// controller.activate(Action::MoveRight);
    /// // active[Action::MoveRight] == 0
    /// controller.update();
    /// // active[Action::MoveRight] == 1
    /// controller.update();
    /// controller.update();
    /// // active[Action::MoveRight] == 3
    /// controller.deactivate(Action::MoveRight);
    /// controller.update();
    /// // active[Action::MoveRight] == 0
    /// ```
    pub fn update(&mut self) {
        for i in 0..self.active.len() {
            self.time[i] = if self.active[i] { self.time[i] + 1 } else { 0 };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut controller = Controller::new();

        controller.activate(Action::MoveLeft);
        assert_eq!(controller.active[Action::MoveLeft as usize], true);
        assert_eq!(controller.time[Action::MoveLeft as usize], 0);

        controller.update();
        assert_eq!(controller.time[Action::MoveLeft as usize], 1);

        controller.deactivate(Action::MoveLeft);
        assert_eq!(controller.time[Action::MoveLeft as usize], 1);

        controller.update();
        assert_eq!(controller.time[Action::MoveLeft as usize], 0);

        controller.activate(Action::MoveLeft);
        controller.activate(Action::MoveRight);
        controller.update();
        controller.update();
        controller.update();
        assert_eq!(controller.time[Action::MoveLeft as usize], 3);
        assert_eq!(controller.time[Action::MoveRight as usize], 3);
    }
}
