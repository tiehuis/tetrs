//! Implements a high-level engine which composes all the components
//! into one abstract structure.

#![allow(dead_code)]

use field::{Field, FieldOptions};
use controller::{Action, Controller};
use block::{self, Rotation, Direction, Block, BlockOptions};
use randomizer::{self, Randomizer, BagRandomizer};
use rotation_system::{self, RotationSystem};
use wallkick::{self, Wallkick};
use utility::BlockHelper;
use statistics::Statistics;
use history::History;

use time;
use collections::enum_set::CLike;
use std::io::prelude::*;
use std::fs::File;
use serde_json;

/// Stores a number of internal options that may be useful during a games
/// execution.
///
/// Unlike `NullpoMino` these are never tied to a name. This can be handled by
/// the caller if required.
///
/// Currently `Options` do not manage the randomizer/rotation system etc. Need
/// to determine exactly what is required.
#[derive(Serialize, Deserialize, Debug)]
#[allow(missing_docs)]
pub struct EngineOptions {
    pub field_options: FieldOptions,

    pub randomizer_name: String,

    pub randomizer_lookahead: usize,

    pub rotation_system_name: String,

    pub wallkick_name: String,

    pub mspt: u64,

    pub engine_settings: EngineSettings
}

impl Default for EngineOptions {
    fn default() -> EngineOptions {
        EngineOptions {
            field_options: FieldOptions { ..Default::default() },
            randomizer_name: "bag".to_string(),
            randomizer_lookahead: 7,
            rotation_system_name: "srs".to_string(),
            wallkick_name: "srs".to_string(),
            mspt: 16,
            engine_settings: EngineSettings { ..Default::default() }
        }
    }
}

impl EngineOptions {
    /// Construct an `EngineOptions` from a file.
    pub fn load_file(name: &str) -> EngineOptions {
        let mut f = match File::open(name) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open file: {}", e)
        };
        let mut buffer = String::new();
        let _ = f.read_to_string(&mut buffer);
        serde_json::from_str(&buffer).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Settings used internally by an `Engine`.
pub struct EngineSettings {
    /// DAS setting (in ms)
    pub das: u64,

    /// ARE time (in ms)
    pub are: u64,

    /// Gravity (in ms). How many ms must pass for block to fall
    pub gravity: u64,

    /// Auto-repeat-rate (in ms)
    pub arr: u64,

    /// Is hold enabled
    pub has_hold: bool,

    /// How many times can we hold
    pub hold_limit: u64,

    /// Is hard drop allowed
    pub has_hard_drop: bool,

    /// Has soft drop
    pub has_soft_drop: bool,

    /// The speed soft drop works
    pub soft_drop_speed: u64,

    /// Maximum number of preview pieces
    pub preview_count: u64
}

impl Default for EngineSettings {
    fn default() -> EngineSettings {
        EngineSettings {
            das: 180,
            are: 0,
            gravity: 1000,
            arr: 16,
            has_hold: true,
            hold_limit: 1,
            has_hard_drop: true,
            has_soft_drop: true,
            soft_drop_speed: 16, // 1G
            preview_count: 3
        }
    }
}

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
struct EngineState {
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
    pub rotation_system: &'static RotationSystem,

    /// The field which the game is played on
    pub field: Field,

    /// The active block
    pub block: Block,

    /// The current hold block (this doesn't store an actual block right now)
    pub hold: Option<block::Id>,

    /// Settings used internally by the engine
    pub options: EngineSettings,

    /// Statistics of the current game
    pub statistics: Statistics,

    /// The input history of the game
    pub history: History,

    /// Is the game running
    pub running: bool,

    /// How many milliseconds occur per game tick.
    pub mspt: u64,

    /// How many ticks have elapsed this game
    pub tick_count: u64,

    /// Private internal state flags
    internal: EngineState,

    /// The current game status. There are 5 main states that are utilized:
    /// - Ready     -> Triggers for the first 50 frames
    /// - Go        -> Triggers for the next 50 frames
    /// - Move      -> Main state for when the game is running
    /// - GameOver  -> Occurs on game failure
    /// - Excellent -> Occurs on goal reached
    status: Status
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
    pub fn new(options: EngineOptions) -> Engine {
        let mut engine = Engine {
            field: Field::with_options(options.field_options),
            randomizer: randomizer::new(&options.randomizer_name, options.randomizer_lookahead),
            controller: Controller::new(),
            rotation_system: rotation_system::new(&options.rotation_system_name),
            wallkick: wallkick::new(&options.wallkick_name),
            block: Block { id: block::Id::None, x: 0, y: 0, r: Rotation::R0, rs: rotation_system::new("srs") },
            hold: None,
            tick_count: 0,
            mspt: options.mspt,
            running: true,
            options: options.engine_settings,
            history: History::new(),
            statistics: Statistics::new(),
            internal: EngineState { ..Default::default() },
            status: Status::Ready,
        };

        // Can we utilize internal data when constructing the block?
        engine.block = Block::with_options(engine.randomizer.next(), &engine.field,
            BlockOptions { rotation_system: engine.rotation_system, ..Default::default() }
        );
        engine
    }

    /// Adjusts a constant value to ticks for the current gamestate.
    ///
    /// This takes a self argument, and the value to convert. If macro is
    /// seperated by a ';' instead of a ',', the second argument is treated
    /// as an identifier to a member function of `self`. This is cuts down
    /// on repeated self references in a single call.
    ///
    /// ```text
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
        self.history.update(&self.controller);

        // Calculate movement then gravity or gravity then movement?

        if self.controller.active(Action::MoveLeft) && self.controller.active(Action::MoveRight) {
            let action = if self.controller.time(Action::MoveLeft) < self.controller.time(Action::MoveRight) {
                Direction::Left
            } else {
                Direction::Right
            };

            if self.controller.time(Action::MoveLeft) > self.ticks(self.options.das) ||
                self.controller.time(Action::MoveRight) > self.ticks(self.options.das) {
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
            if (down - 1) % self.ticks(self.options.soft_drop_speed) == 0 {
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
                self.block = Block::with_options(self.randomizer.next(), &self.field,
                    BlockOptions { rotation_system: self.rotation_system, ..Default::default() }
                );
            }
            else {
                let tmp = self.block.id;
                self.block = Block::with_options(self.hold.unwrap(), &self.field,
                    BlockOptions { rotation_system: self.rotation_system, ..Default::default() }
                );
                self.hold = Some(tmp);
            }
        }

        // Handle hard drop
        if self.controller.time(Action::HardDrop) == 1 {
            self.block.shift_extend(&self.field, Direction::Down);
            self.field.freeze(self.block.clone());
            self.block = Block::with_options(self.randomizer.next(), &self.field,
                BlockOptions { rotation_system: self.rotation_system, ..Default::default() }
            );

            self.internal.hold_count = 0;
            self.statistics.pieces += 1;
        }

        // Clear all line
        let cleared = self.field.clear_lines();
        self.statistics.lines += cleared as u64;

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

        // Perform gravity
        if self.tick_count % self.ticks(self.options.gravity) == 0 {
            self.block.shift(&self.field, Direction::Down);
        }
    }

    fn update_gameover(&self) {
    }

    fn update_excellent(&self) {
    }
}

#[cfg(test)]
mod tests {
    use ::import::*;

    #[test]
    fn test_engine() {
        let _ = Engine::new(EngineOptions { ..Default::default() });
    }
}
