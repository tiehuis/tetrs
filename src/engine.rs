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
use options::Options;
use statistics::Statistics;

use time;
use collections::enum_set::CLike;

/// Which part of the game are we in. This is used to keep track of multi-frame
/// events that require some internal state past state to be kept.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Status {
    None, Setting, Ready, Move, LockFlash, LineClear, Are, EndingStart,
    Excellent, GameOver
}

/// Stores variables which are modified internally during execution. Namely
/// state counters and the like. This is namespaced to have better modularity and
/// general code structure.
///
/// This is required for state that is not reliant on frame counts.
#[derive(Default)]
struct InternalEngineState {
    /// Last update invocation time
    last_update_time: u64,

    /// How many successive holds have been made
    hold_count: u64
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

    /// The current hold block (this doesn't store an actual block right now)
    pub hold: Option<block::Type>,

    /// Immutable game options
    pub options: Options,

    /// Statistics of the current game
    pub statistics: Statistics,

    /// Is the game running
    pub running: bool,

    /// How many milliseconds occur per game tick.
    pub mspt: u64,

    /// How many ticks have elapsed this game
    pub tick_count: u64,

    /// Private internal state flags
    internal: InternalEngineState,

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
            tick_count: 0,
            mspt: 16,
            running: true,
            options: Options::new(),
            statistics: Statistics::new(),
            internal: InternalEngineState { ..Default::default() },
            status: Status::Ready
        };

        // Cannot initialize in struct due to lifetime problems
        // TODO: Use engine rotation on block initialization
        let block = Block::new(engine.randomizer.next())
                          .set_field(&engine.field)
                          .set_rotation_system(rotation::SRS::new());
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
    pub fn new(options: Options) -> Engine {
        Engine { options: options, ..Default::default() }
    }

    /// Adjusts a constant value to ticks for the current gamestate.
    ///
    /// This takes a self argument, and the value to convert. If macro is
    /// seperated by a ';' instead of a ',', the second argument is treated
    /// as an identifier to a member function of `self`. This is cuts down
    /// on repeated self references in a single call.
    ///
    /// ```ignore
    /// let ticks_to_wait = self.ticks(self, 789);
    ///
    /// // Access the options.das field and convert to ticks
    /// let ticks_to_wait = self.ticks(self; options.das);
    /// ```
    fn ticks(&self, val: u64) -> u64 {
        val / self.mspt
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

        self.tick_count += 1;
    }

    /// This is the initial `countdown` and is called for the first
    /// 1666ms of play.
    fn update_ready(&mut self) {
        // Allow DAS charging and initial hold

        match self.tick_count {
            x if x == self.ticks(0)    => self.status = Status::Move,
            x if x == self.ticks(833)  => (),
            x if x == self.ticks(1667) => self.status = Status::Move,
            _ => ()
        }
    }

    fn is_pressed(&self, action: Action, rate: u64) -> bool {
        let sct = self.controller.time[action.to_usize()] as u64;
        let das = self.ticks(self.options.das);

        // First press, or over das and arr rate has fired
        sct == 1 || (sct >= das && (sct - das) % self.ticks(rate) == 0)
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

            if self.controller.time(Action::MoveLeft) > self.ticks(self.options.das) as usize ||
                self.controller.time(Action::MoveRight) > self.ticks(self.options.das) as usize {
                self.block.shift(&self.field, action);
            }
        }

        if self.is_pressed(Action::MoveLeft, self.options.arr) {
            self.block.shift(&self.field, Direction::Left);
        }

        if self.is_pressed(Action::MoveRight, self.options.arr) {
            self.block.shift(&self.field, Direction::Right);
        }

        // Drop has no DAS and is immediate
        if self.controller.active(Action::MoveDown) {
            let down = self.controller.time(Action::MoveDown);
            if (down - 1) % self.ticks(self.options.soft_drop_speed) as usize == 0 {
                self.block.shift(&self.field, Direction::Down);
            }
        }

        // Handle rotations
        if self.controller.time(Action::RotateLeft) == 1 {
            self.block.rotate_with_wallkick(&self.field, self.wallkick, Rotation::R270);
        }
        if self.controller.time(Action::RotateRight) == 1 {
            self.block.rotate_with_wallkick(&self.field, self.wallkick, Rotation::R90);
        }

        // Hold currently only stores block id. Would be interesting to store the block
        // and hold could possibly save the state of the blocks rotation?
        if self.controller.time(Action::Hold) == 1 && self.internal.hold_count < self.options.hold_limit {
            self.internal.hold_count += 1;
            if self.hold.is_none() {
                self.hold = Some(self.block.id);
                self.block = Block::new(self.randomizer.next())
                                   .set_field(&self.field);
            }
            else {
                let tmp = self.block.id;
                self.block = Block::new(self.hold.unwrap())
                                   .set_field(&self.field);
                self.hold = Some(tmp);
            }
        }

        // Handle hard drop
        if self.controller.time(Action::HardDrop) == 1 {
            self.block.shift_extend(&self.field, Direction::Down);
            self.field.freeze(self.block.clone());
            self.block = Block::new(self.randomizer.next())
                               .set_field(&self.field)
                               .set_rotation_system(rotation::SRS::new());
            self.internal.hold_count = 0;
        }

        // Clear all line
        let cleared = self.field.clear_lines();
        self.statistics.lines += cleared as u32;

        match cleared {
            4 => self.statistics.fours += 1,
            3 => self.statistics.triples += 1,
            2 => self.statistics.doubles += 1,
            1 => self.statistics.singles += 1,
            _ => ()
        };

        if self.controller.time(Action::Quit) == 1 {
            self.running = false;
        }

        // Determine if we are lagging or being called too early
        // If outside of 5% error, warn. Note: should warn only once, or
        // limited.
        if self.internal.last_update_time == 0 {
            self.internal.last_update_time = time::precise_time_ns();
        }
        else {
            let now = time::precise_time_ns();
            let rate = (now - self.internal.last_update_time) as f64 / (self.mspt * 1_000_000) as f64;

            if rate > 1.05 {
                warn!("Update lagging! {}% of expected update time", rate);
            }
            else if rate < 0.95 {
                warn!("Update called too quick! {}% of expected update time", rate);
            }

            self.internal.last_update_time = now;
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
    use options::Options;

    #[test]
    fn test_engine() {
        let _ = Engine::new(Options::new());
    }
}
