//! Implements a high-level engine which composes all the components
//! into one abstract structure.

#![allow(dead_code)]

use field::{Field, FieldBuilder};
use controller::{Action, Controller};
use block::{self, Rotation, Direction, Block, BlockBuilder};
use randomizer::{Randomizer, BagRandomizer};
use rotation::{self, RotationSystem};
use wallkick::{self, Wallkick};
use utility::BlockHelper;

/// Which part of the game are we in. This is used to keep track of multi-frame
/// events that require some internal state past state to be kept.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Status {
    None, Setting, Ready, Move, LockFlash, LineClear, Are, EndingStart,
    Excellent, GameOver
}


/// This engine allows for handling of DAS-like features and other things
/// which are otherwise transparent to sub-components which are only
/// managed on a per-tick basis (have no concept of state over time).
pub struct Engine {
    /// Controller which is used by the engine
    pub controller: Controller,

    /// The randomizer being used.
    pub randomizer: BagRandomizer,

    /// The wallkick object being used.
    pub wallkick: &'static Wallkick,

    /// The rotation system used by this engine.
    pub rs: &'static RotationSystem,

    /// The field which the game is played on
    pub field: Field,

    /// The active block
    pub block: Block,

    /// The current hold block
    pub hold: Option<Block>,

    /// Das value
    pub das: usize,

    /// Is the game running
    pub running: bool,

    /// How many milliseconds occur per game tick.
    pub mspt: u64,

    /// How many ticks have elapsed this game
    pub ticks: u64,

    /// The current game status. There are 5 main states that are utilized:
    /// - Ready     -> Triggers for the first 50 frames
    /// - Go        -> Triggers for the next 50 frames
    /// - Move      -> Main state for when the game is running
    /// - GameOver  -> Occurs on game failure
    /// - Excellent -> Occurs on goal reached
    status: Status
}

impl Default for Engine {
    fn default() -> Engine {
        let mut engine = Engine {
            randomizer: BagRandomizer::new(7),
            controller: Controller::new(),
            rs: rotation::SRS::new(),
            wallkick: wallkick::SRS::new(),
            field: Field::new().set_hidden(2),
            block: Block::new(block::Type::None),
            hold: None,
            ticks: 0,
            mspt: 16,
            das: 150,
            running: true,
            status: Status::Ready
        };

        // Cannot initialize in struct due to lifetime problems
        let block = Block::new(engine.randomizer.next())
                          .set_field(&engine.field)
                          .set_rotation_system(rotation::DTET::new());
        engine.block = block;
        engine
    }
}

impl Engine {

    /// Construct a new engine object and return it.
    ///
    /// This should be used as a compositional object of all the underlying
    /// objects. This adheres moreso to the rust philosophy and gives greater
    /// variance in how an engine can be constructed.
    ///
    /// An engine is constructed in an initialized state and is ready to be
    /// used right from the beginning.
    pub fn new() -> Engine {
        Engine { ..Default::default() }
    }

    /// Adjusts a constant value to ticks for the current gamestate.
    ///
    /// i.e. We know that the Ready state must last for 50 frames (at 60fps),
    /// but since we can vary the `mspt`, this requires a conversion function.
    ///
    /// ```ignore
    /// let ticks_to_wait = ms_to_ticks(834);
    /// ```
    #[inline]
    fn ms_to_ticks(&self, ms: u64) -> u64 {
        ms / self.mspt
    }

    /// Return the current left-right move direction. Since both actions can
    /// occur simultaneously there are a number of different behaviours here
    /// that could be set.
    fn lr_move_direction(&self) -> Option<Direction> {
        let sc = &self.controller;
        match (sc.active(Action::MoveLeft), sc.active(Action::MoveRight)) {
            (true, true) => {
                if sc.time(Action::MoveLeft) > sc.time(Action::MoveRight) {
                    Some(Direction::Right)
                } else {
                    Some(Direction::Left)
                }
            },
            (true, false) => Some(Direction::Left),
            (false, true) => Some(Direction::Right),
            (false, false) => None
        }
    }

    /// Convert a left-right direction to an action
    fn d2a(direction: Direction) -> Option<Action> {
        match direction {
            Direction::Left => Some(Action::MoveLeft),
            Direction::Right => Some(Action::MoveRight),
            _ => None
        }
    }

    /// The main update phase of the engine.
    ///
    /// This handles DAS and all other internal complications based on the
    /// current controller state only.
    ///
    /// Each call to update is expected to take place in `~mspt` ms. It
    /// is up to the caller to manage the update lengths appropriately.
    pub fn update(&mut self) {
        self.controller.update();

        match self.status {
            Status::Ready => self.update_ready(),
            Status::Move => self.update_move(),
            Status::GameOver => self.update_gameover(),
            Status::Excellent => self.update_excellent(),
            x => panic!("Cannot handle status {:?}", x)
        }

        self.ticks += 1;
    }

    /// This is the initial `countdown` and is called for the first
    /// 1666ms of play.
    fn update_ready(&mut self) {
        // Allow DAS charging and initial hold

        match self.ticks {
            x if x == self.ms_to_ticks(0)    => self.status = Status::Move,
            x if x == self.ms_to_ticks(833)  => (),
            x if x == self.ms_to_ticks(1667) => self.status = Status::Move,
            _ => ()
        }
    }

    /// This performs the bulk of the gameplay logic.
    fn update_move(&mut self) {
        // Calculate movement then gravity or gravity then movement?

        if self.controller.active(Action::MoveLeft) && self.controller.active(Action::MoveRight) {
            let action = if self.controller.time(Action::MoveLeft) < self.controller.time(Action::MoveRight) {
                Direction::Left
            } else {
                Direction::Right
            };

            if self.controller.time(Action::MoveLeft) > self.ms_to_ticks(self.das as u64) as usize ||
                self.controller.time(Action::MoveRight) > self.ms_to_ticks(self.das as u64) as usize {
                self.block.shift(&self.field, action);
            }
        }

        if self.controller.time(Action::MoveLeft) == 1 ||
            self.controller.time(Action::MoveLeft) > self.ms_to_ticks(self.das as u64) as usize {
            self.block.shift(&self.field, Direction::Left);
        }

        if self.controller.time(Action::MoveRight) == 1 ||
            self.controller.time(Action::MoveRight) > self.ms_to_ticks(self.das as u64) as usize {
            self.block.shift(&self.field, Direction::Right);
        }

        if self.controller.time(Action::MoveDown) == 1 ||
            self.controller.time(Action::MoveDown) > self.ms_to_ticks(self.das as u64) as usize {
            self.block.shift(&self.field, Direction::Down);
        }

        // Handle rotations
        if self.controller.time(Action::RotateLeft) == 1 {
            self.block.rotate_with_wallkick(&self.field, self.wallkick, Rotation::R270);
        }
        if self.controller.time(Action::RotateRight) == 1 {
            self.block.rotate_with_wallkick(&self.field, self.wallkick, Rotation::R90);
        }

        // Handle hold: Error here generating new blocks too much
        /*
        match self.hold.clone() {
            Some(hold) => {
                // TODO: May need a temporary here depending on binding
                self.hold = Some(self.block.clone());
                self.block = Block::new(hold.id).set_field(&self.field);
            },
            None => {
                self.hold = Some(self.block.clone());
                self.block = Block::new(self.randomizer.next())
                                   .set_field(&self.field)
                                   .set_rotation(Rotation::R0);
            }
        };
        */

        // Handle hard drop
        if self.controller.time(Action::HardDrop) == 1 {
            self.block.shift_extend(&self.field, Direction::Down);
            self.field.freeze(self.block.clone());
            self.block = Block::new(self.randomizer.next())
                               .set_field(&self.field)
                               .set_rotation_system(rotation::DTET::new());
        }

        // Clear all lines
        self.field.clear_lines();

        if self.controller.time(Action::Quit) == 1 {
            self.running = false;
        }
    }

    fn update_gameover(&self) {
    }

    fn update_excellent(&self) {
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_engine() {
        let _ = Engine::new();
    }
}
