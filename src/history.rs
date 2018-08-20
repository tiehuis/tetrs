//! Module which provides history management tools.

use controller::{Action, Controller, CAarray};

/// An individual event in a history sequence.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Event {
    /// Was the event a press or release?
    press: bool,

    /// At what tick did this event occur
    ticks: u64,

    /// What action does this event represent
    action: Action
}

/// Manages the history state of a particular game.
///
/// This at its simplest is a sequence of `(u64, Action)` tuples which
/// represent particular the entire game input state.
///
/// This can be used to recreate games or provide other interesting
/// statistics. This at some point could be combined with the statistics
/// class, where statistic could be some `Event`-like structure which
/// would allow time-based statistics tracking.
#[derive(Default)]
pub struct History {
    /// Ordered sequence of historical events
    history: Vec<Event>,

    /// The last snapshot of what actions were active in the controller
    snapshot: CAarray,

    /// Current tick count
    tick_count: u64
}

impl History {
    /// Construct a new empty history sequence
    pub fn new() -> History {
        History { ..Default::default() }
    }

    /// Update the history state with a controller snapshot.
    pub fn update(&mut self, controller: &Controller) {
        for (i, (last, curr)) in self.snapshot.iter()
                                     .zip(controller.active.iter())
                                     .enumerate() {
            match (*last, *curr) {
                (true, false) => {
                    self.history.push(Event {
                        press: false,
                        ticks: self.tick_count,
                        action: Action::from(i)
                    });
                },
                (false, true) => {
                    self.history.push(Event {
                        press: true,
                        ticks: self.tick_count,
                        action: Action::from(i)
                    });
                },
                _ => ()
            }
        }

        self.tick_count += 1;
    }

    /// Return the current event sequence
    pub fn get_sequence(&self) -> &[Event] {
        &self.history
    }
}
