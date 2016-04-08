//! An abstract controller for specifying actions.
//!
//! This controller acts like a bridge between any physical input and the
//! input which the internal gamestate can understand.
//!
//! When a key is pressed, this can be translated to a call to the controllers
//! `activate` function, and when the key is removed, this corresponds to a
//! call to the `deactivate` function.

use std::mem;
use collections::enum_set::CLike;

/// Actions which are understood by the controller.
#[repr(usize)]
#[derive(Clone, Copy, Debug, Hash)]
#[allow(missing_docs)]
pub enum Action {
    MoveLeft, MoveRight, MoveDown, HardDrop,
    RotateLeft, RotateRight, Hold, Quit
}

impl CLike for Action {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(v: usize) -> Action {
        unsafe { mem::transmute(v) }
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
    time: [usize; 8],

    /// Which actions are currently active.
    active: [bool; 8]
}

impl Controller {
    /// Return a new controller instance.
    ///
    /// The controller specified will start with all actions in the inactive
    /// state.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::controller;
    ///
    /// let mut controller = controller::Controller::new();
    /// ```
    pub fn new() -> Controller {
        Controller { time: [0; 8], active: [false; 8] }
    }

    /// Query if an action is currently active.
    pub fn active(&self, action: Action) -> bool {
        self.active[action.to_usize()]
    }

    /// Query how long an action has been active for.
    pub fn time(&self, action: Action) -> usize {
        self.time[action.to_usize()]
    }

    /// Activate the specified action.
    ///
    /// The action will be set to active and all subsequent updates will
    /// now trigger this actions timer.
    /// Reactivating an already active action has no effect and will not
    /// restart the timer.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::controller;
    ///
    /// let mut controller = controller::Controller::new();
    /// controller.activate(controller::Action::MoveLeft);
    /// ```
    pub fn activate(&mut self, action: Action) {
        self.active[action.to_usize()] = true;
    }

    /// Deactivate the specified action.
    ///
    /// The action will be set to inactive.
    /// Deactivating an already inactive action has no effect.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::controller;
    ///
    /// let mut controller = controller::Controller::new();
    /// controller.deactivate(controller::Action::MoveLeft);
    /// ```
    pub fn deactivate(&mut self, action: Action) {
        self.active[action.to_usize()] = false;
    }

    /// Deactivate all actions.
    ///
    /// This is useful when calculating actions based on the state of
    /// an object, rather than explicitly via events.
    ///
    /// This does not reset the internal time of each action to 0.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::controller::{Controller, Action};
    ///
    /// let mut controller = Controller::new();
    /// controller.deactivate_all();
    /// ```
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
    use collections::enum_set::CLike;

    #[test]
    fn test() {
        let mut controller = Controller::new();

        controller.activate(Action::MoveLeft);
        assert_eq!(controller.active[Action::MoveLeft.to_usize()], true);
        assert_eq!(controller.time[Action::MoveLeft.to_usize()], 0);

        controller.update();
        assert_eq!(controller.time[Action::MoveLeft.to_usize()], 1);

        controller.deactivate(Action::MoveLeft);
        assert_eq!(controller.time[Action::MoveLeft.to_usize()], 1);

        controller.update();
        assert_eq!(controller.time[Action::MoveLeft.to_usize()], 0);

        controller.activate(Action::MoveLeft);
        controller.activate(Action::MoveRight);
        controller.update();
        controller.update();
        controller.update();
        assert_eq!(controller.time[Action::MoveLeft.to_usize()], 3);
        assert_eq!(controller.time[Action::MoveRight.to_usize()], 3);
    }
}
