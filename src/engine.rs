//! Implements a high-level engine which composes all the components
//! into one abstract structure.

// engine.rs (revised)
//
// This attempts to go over a more extensible way of expanding the `Engine`
// class with the required features. This is just one big stateful instance and
// is a bit unwieldy, although its a suitable method to use.
//
// The first revision should have enough support to handle DAS, ARR, ARE, IHS,
// IRS well.
//
// (omitted real IRS and IHS handling)

use collections::enum_set::CLike;

use block::{self, Block, BlockOptions, Rotation, Direction};
use field::{Field, FieldOptions};
use controller::{Controller, Action};
use randomizer::{self, Randomizer};
use wallkick::{self, Wallkick};
use statistics::Statistics;
use history::History;
use utility::BlockHelper;
use rotation_system::{self, RotationSystem};

/// The current `Engine` status.
#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    /// Entry delay for piece spawn
    Are,

    /// Main movement phase
    Move,

    /// Occurs on lockout or game failure
    GameOver,

    /// Default status indicating nothing should happen
    None
}
impl Default for Status { fn default() -> Status { Status::None } }


/// Stores internal `Engine` status flags.
///
/// When adding new values, these should default to the standard defaults
/// for primitives.
#[derive(Default)]
struct EngineInternal {
    /// How many ticks have we been in the current status
    status_timer: u64,

    /// Current gravity of the piece
    gravity_counter: f64,

    /// The current soft drop frames to move
    soft_drop_counter: f64,

    /// How many times the current piece has been held
    hold_count: u64,

    /// Is the piece currently locking
    locking: bool,

    /// How long has the piece been locking
    lock_timer: u64,

    /// Was an Initial Hold requested?
    ihs_flag: bool,

    /// Was an Initial Rotate requested?
    irs_flag: bool,

    /// Which rotation direction was requested?
    irs_rotation: Rotation,

    /// Do we need a new piece spawned?
    need_piece: bool,

    /// How long has the current piece been alive?
    piece_timer: u64,
}


/// Stores configurable options which alter how the engine works.
pub struct EngineSettings {
    /// How many ms should are last for
    are: u64,

    /// Auto-repeat-rate (in ms)
    arr: u64,

    /// Delayed auto-shift (in ms)
    das: u64,

    /// How fast soft drop occurs (cells per ms)
    soft_drop_speed: f64,

    /// How long the lock delay exists for
    lock_delay: u64,

    /// How many times can we hold per block
    hold_limit: u64,

    /// How many frames moved per ms
    gravity: f64,

    /// Should gravity be performed before move?
    gravity_before_move: bool,
}

impl Default for EngineSettings {
    fn default() -> EngineSettings {
        EngineSettings {
            are: 0, arr: 16, das: 180, soft_drop_speed: 2f64,
            lock_delay: 300, hold_limit: 1, gravity: 0.001,
            gravity_before_move: false
        }
    }
}


/// Struct for initializing an `Engine`
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


/// Stores the internal engine details.
///
/// This is largely segmented into components `EngineSettings`, `EngineInternal`
/// to reduce the overall complexity and namespace various features.
///
/// Most names are condensed a lot to provide shorter references.
pub struct Engine {
    /// Controller which is used by the engine
    pub co: Controller,

    /// The randomizer being used.
    pub rd: Box<Randomizer>,

    /// The wallkick object being used.
    pub wk: &'static Wallkick,

    /// The rotation system used by this engine.
    pub rs: &'static RotationSystem,

    /// The field which the game is played on
    pub fd: Field,

    /// The active block
    pub bk: Block,

    /// The current hold block (this doesn't store an actual block right now)
    pub hd: Option<block::Id>,

    /// Settings used internally by the engine
    pub op: EngineSettings,

    /// Statistics of the current game
    pub st: Statistics,

    /// The input history of the game
    pub hs: History,

    /// Is the game running
    pub running: bool,

    /// How many milliseconds occur per game tick.
    pub mspt: u64,

    /// How many ticks have elapsed this game
    pub tick_count: u64,

    /// Private internal state flags
    it: EngineInternal,

    /// The current game status.
    status: Status,

    /// Status of the last frame
    last_status: Status
}

impl Engine {
    /// Adjusts a constant value to ticks for the current gamestate.
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

    /// Check if a particular action has been pressed with the specified
    /// rate.
    fn is_pressed(&self, action: Action, rate: u64) -> bool {
        let sct = self.co.time[action.to_usize()] as u64;
        let das = self.ticks(self.op.das);

        // First press, or over das and arr rate has fired
        sct == 1 || (sct >= das && (sct - das) % self.ticks(rate) == 0)
    }

    /// The main update phase of the engine.
    ///
    /// This handles DAS and all other internal complications based on the
    /// current controller state only.
    ///
    /// Each call to update is expected to take place in `~mspt` ms. It
    /// is up to the caller to manage the update lengths appropriately.
    pub fn update(&mut self) {
        self.co.update();
        self.last_status = self.status;

        if self.it.need_piece {
            self.do_piece_spawn();
            self.it.need_piece = false;

            // Have a method to reset all block internal counts
            self.it.locking = false;
            self.it.piece_timer = 0;
            self.it.hold_count = 0;
            self.it.lock_timer = 0;
            self.it.soft_drop_counter = 0f64;
            self.it.gravity_counter = 0f64;
        }

        match self.status {
            Status::Move => self.stat_move(),
            Status::Are => self.stat_are(),
            Status::GameOver => self.stat_gameover(),
            Status::None => ()
        }

        // If the status changed during processing, reset timers
        if self.status != self.last_status {
            self.it.status_timer = 0;
        }
        else {
            self.it.status_timer += 1;
        }
    }

    /// High-level move function. This should be easy enough to follow.
    fn stat_move(&mut self) {
        // Handle Initial state change on first frame.
        if self.it.piece_timer == 0 {
            // Initial hold can be activated in other statuses and preserved, so we
            // cannot just check for a hold keypress.
            if self.it.ihs_flag {
                self.do_hold();
                self.it.ihs_flag = false;
            }

            // Perform a rotation before the piece is spawn (tested for collision).
            // Likewise with IHS, we cannot just check for a rotate key press.
            //
            // We can perform a double rotation with IRS on the first frame.
            // This is valid behavior.,
            if self.it.irs_flag {
                self.bk.rotate_with_wallkick(&self.fd, self.wk, self.it.irs_rotation);
                self.it.irs_flag = false;
                self.it.irs_rotation = Rotation::R0;
            }

            // We only check for a complete lockout on the first frame the piece spawned.
            // If we have an overlap, then this is invalid and the game is over.
            if self.check_lockout() {
                self.status = Status::GameOver;
                return;
            }
        }

        // Hard drop has max priority and overrides any other moves
        if !self.check_hard_drop() {
            // Check for a hold action and perform it if present
            self.check_hold();

            // Check for a rotate action and perform it if present
            self.check_rotate();

            // Would be nice to have this option
            if self.op.gravity_before_move {
                self.check_gravity_and_soft_drop();
                self.check_move();
            }
            else {
                self.check_move();
                self.check_gravity_and_soft_drop();
            }
        }

        // Check lockout once more, this may alter the current state if the block
        // is deemed as locking.
        self.check_lock();

        // Check line clear
        self.fd.clear_lines();

        // Update the current piece timer
        self.it.piece_timer += 1;
    }


    /// Perform ARE frame
    fn stat_are(&mut self) {
        // Check for initial rotate/hold

        // Check for are cancel

        if self.it.status_timer > self.ticks(self.op.are) {
            self.it.need_piece = true;
            self.status = Status::Move;
        }
    }

    /// Perform game over phase
    fn stat_gameover(&mut self) {
        self.running = false;
    }

    /// Perform a hold, swapping the current piece with the hold piece.
    ///
    /// If no hold piece is set, then the piece is taken from the randomizer.
    fn do_hold(&mut self) {
        if self.hd.is_none() {
            self.hd = Some(self.bk.id);
            self.bk = Block::with_options(self.rd.next(), &self.fd,
                BlockOptions { rotation_system: self.rs, ..Default::default() }
            );
        }
        else {
            let tmp = self.bk.id;
            self.bk = Block::with_options(self.hd.unwrap(), &self.fd,
                BlockOptions { rotation_system: self.rs, ..Default::default() }
            );
            self.hd = Some(tmp);
        }
    }

    /// Retrieve the next piece from the bag and set the current piece to this.
    fn do_piece_spawn(&mut self) {
        self.bk = Block::with_options(self.rd.next(), &self.fd,
            BlockOptions { rotation_system: self.rs, ..Default::default() }
        );
    }

    /// Check for a lockout with the current piece.
    fn check_lockout(&mut self) -> bool {
        self.bk.collides(&self.fd)
    }

    /// Check if a hold action is present and if so try to perform a hold.
    fn check_hold(&mut self) -> bool {
        if self.co.time(Action::Hold) == 1 && self.it.hold_count < self.op.hold_limit {
            self.do_hold();
            self.it.hold_count += 1;
            true
        }
        else {
            false
        }
    }

    /// Check if a movement action is present and perform movement.
    /// TODO: Ensure when moving while locking we readjust internal lock
    /// counter so we don't get floating mid-air blocks!
    fn check_move(&mut self) -> bool {
        if self.co.active(Action::MoveLeft) && self.co.active(Action::MoveRight) {
            let action = if self.co.time(Action::MoveLeft) < self.co.time(Action::MoveRight) {
                Direction::Left
            }
            else {
                Direction::Right
            };

            if self.co.time(Action::MoveLeft) > self.ticks(self.op.das) ||
                    self.co.time(Action::MoveRight) > self.ticks(self.op.das) {
                self.bk.shift(&self.fd, action);
            }

            true
        }
        else if self.is_pressed(Action::MoveLeft, self.op.arr) {
            self.bk.shift(&self.fd, Direction::Left);
            true
        }
        else if self.is_pressed(Action::MoveRight, self.op.arr) {
            self.bk.shift(&self.fd, Direction::Right);
            true
        }
        else {
            false
        }
    }

    /// Check if a rotation action is present and perform it.
    fn check_rotate(&mut self) -> bool {
        let mut r = false;
        if self.co.time(Action::RotateLeft) == 1 {
            self.bk.rotate_with_wallkick(&self.fd, self.wk, Rotation::R270);
            r = true;
        }
        if self.co.time(Action::RotateRight) == 1 {
            self.bk.rotate_with_wallkick(&self.fd, self.wk, Rotation::R90);
            r = true;
        }

        r
    }

    /// Check if a hard drop action is present and if so perform it.
    ///
    /// This does not perform the locking of the piece, this is managed by
    /// the `check_lock` function.
    fn check_hard_drop(&mut self) -> bool {
        if self.co.time(Action::HardDrop) == 1 {
            self.bk.shift_extend(&self.fd, Direction::Down);
            true
        }
        else {
            false
        }
    }

    /// Check the current gravity and perform any downward movement required.
    ///
    /// This may move multiple times per frame.
    ///
    /// We check gravity with soft drop since there is potential overlap with
    /// which speed we may use (cumulative down movement etc).
    fn check_gravity_and_soft_drop(&mut self) -> bool {
        // op.gravity is how many cells are moved per ms.
        //
        // If we move 1.25 frames per ms, then at 16 frames this is equal
        // to 20 frames (20G)
        self.it.gravity_counter += (self.mspt as f64) * self.op.gravity;

        // Soft drop has an internal counter as well to handle fractional
        // movement correctly.
        if self.co.active(Action::MoveDown) {
            self.it.soft_drop_counter += (self.mspt as f64) * self.op.soft_drop_speed;
        }
        else {
            self.it.soft_drop_counter = 0f64;
        }

        // We decrement both soft drop and gravity at the same time, we only
        // utilize the highest value and do not do cumulative gravity (Option?)
        let mut fell = false;
        while self.it.gravity_counter >= 1f64 || self.it.soft_drop_counter >= 1f64 {
            // Begin lock if we are pushed into floor.
            if !self.bk.shift(&self.fd, Direction::Down) {
                self.it.locking = true;
            }

            if self.it.gravity_counter >= 1f64 {
                self.it.gravity_counter -= 1f64;
            }
            if self.it.soft_drop_counter >= 1f64 {
                self.it.soft_drop_counter -= 1f64;
            }

            // Indicate gravity occurred on this frame
            fell = true;
        }

        fell
    }

    // Check if the current piece should be locked into place.
    //
    // Problem with soft-drop/gravity and lock-delay where pieces are been
    // frozen at the incorrect position.
    fn check_lock(&mut self) {
        let mut instant_lock = false;

        // Hard drop will always lock (if hard drop lock)
        if self.co.time(Action::HardDrop) == 1 {
            // Manual lock on hard drop by default now. This should be an option.
            instant_lock = true;
        }

        // Reset lock timer if over a gap.
        // TODO: Check this does not allow stalling of piece in air.
        if !self.bk.collides_at_offset(&self.fd, (0, 1)) {
            self.it.locking = false;
            self.it.lock_timer = 0;
        }

        // Lock the piece if instant lock or over lock delay.
        // Manage the next state to go to since this block is done.
        if (self.it.lock_timer > self.ticks(self.op.lock_delay)) || instant_lock {
            // Clone is not ideal
            self.fd.freeze(self.bk.clone());

            // Either perform ARE if non-zero, or immediately perform move
            if self.op.are != 0 {
                self.status = Status::Are;
            }
            else {
                // Must explicitly reset status timer for next piece
                self.it.status_timer = 0;
                self.it.need_piece = true;
                self.status = Status::Move;
            }
        }

        // Update the lock delay after we have processed it (0 is first frame).
        if self.it.locking {
            self.it.lock_timer += 1;
        }
    }


    /// Construct a new `Engine` from an `EngineOptions` instance.
    pub fn new(options: EngineOptions) -> Engine {
        let mut engine = Engine {
            fd: Field::with_options(options.field_options),
            rd: randomizer::new(&options.randomizer_name, options.randomizer_lookahead).unwrap(),
            co: Controller::new(),
            rs: rotation_system::new(&options.rotation_system_name).unwrap(),
            wk: wallkick::new(&options.wallkick_name).unwrap(),
            bk: Block { id: block::Id::None, x: 0, y: 0, r: Rotation::R0, rs: rotation_system::new("srs").unwrap() },
            hd: None,
            tick_count: 0,
            mspt: options.mspt,
            running: true,
            op: options.engine_settings,
            hs: History::new(),
            st: Statistics::new(),
            it: EngineInternal { ..Default::default() },
            status: Status::Move,
            last_status: Status::Move
        };

        engine.it.need_piece = true;
        engine
    }
}
