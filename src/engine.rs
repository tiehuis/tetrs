//! Implements a high-level engine which composes all the components
//! into one abstract structure.

use ::{Rotation, Direction};
use ::field::Field;
use ::controller::{Action, Controller};
use ::block::Block;
use ::randomizer::Randomizer;
use ::wallkick::WallkickTest;

/// Which part of the game are we in. This is used to keep track of multi-frame
/// events that require some internal state past state to be kept.
#[derive(Debug, Clone, Copy)]
enum Status {
    None, Setting, Ready, Move, LockFlash, LineClear, Are, EndingStart,
    Excellent, GameOver
}


/// This engine allows for handling of DAS-like features and other things
/// which are otherwise transparent to sub-components which are only
/// managed on a per-tick basis (have no concept of state over time).
pub struct Engine<'a> {
    /// Controller which is used by the engine
    pub controller: Controller,

    /// The randomizer being used.
    pub randomizer: &'a (Randomizer<Item=::block::Type> + 'a),

    /// The wallkick object being used.
    pub wallkick: &'a (WallkickTest + 'a),

    /// The field which the game is played on
    pub field: Field,

    /// The active block
    pub block: Block<'a>,

    /// The current hold block
    pub hold: Option<Block<'a>>,

    /// Das value
    pub das: u64,

    /// How many milliseconds occur per game tick.
    mspt: u64,

    /// How many ticks have elapsed this game
    ticks: u64,

    /// The current game status. There are 5 main states that are utilized:
    /// - Ready     -> Triggers for the first 50 frames
    /// - Go        -> Triggers for the next 50 frames
    /// - Move      -> Main state for when the game is running
    /// - GameOver  -> Occurs on game failure
    /// - Excellent -> Occurs on goal reached
    status: Status
}

impl<'a> Engine<'a> {

    /// Construct a new engine object and return it.
    ///
    /// This should be used as a compositional object of all the underlying
    /// objects. This adheres moreso to the rust philosophy and gives greater
    /// variance in how an engine can be constructed.
    ///
    /// An engine is constructed in an initialized state and is ready to be
    /// used right from the beginning.
    pub fn new(randomizer: &'a Randomizer<Item=::block::Type>, wallkick: &'a WallkickTest, field: Field) -> Engine<'a> {
        Engine {
            randomizer: randomizer,
            controller: Controller::new(),
            wallkick: wallkick,
            field: field,
            block: Block::new(&field, randomizer.next().unwrap(), Rotation::R0),
            hold: None,
            ticks: 0,
            mspt: 16,
            das: 150,
            status: Status::Ready
        }
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
    pub fn update(&'a mut self) {
        self.controller.update();

        match self.status {
            Status::Ready => self.update_ready(),
            Status::Move => self.update_move(),
            Status::GameOver => self.update_gameover(),
            Status::Excellent => self.update_excellent(),
            x @ _ => panic!("Cannot handle status {:?}", x)
        }

        self.ticks += 1;
    }

    /// This is the initial `countdown` and is called for the first
    /// 1666ms of play.
    fn update_ready(&mut self) {
        // Allow DAS charging and initial hold

        match self.ticks {
            x if x == self.ms_to_ticks(0)    => (),
            x if x == self.ms_to_ticks(833)  => (),
            x if x == self.ms_to_ticks(1667) => self.status = Status::Move,
            _ => ()
        }
    }

    /// This performs the bulk of the gameplay logic.
    fn update_move(&'a mut self) {
        match Some(self.lr_move_direction()) {
            None => (),
            direction @ _ => {
                // Handle the left-right movement
                match self.controller.time(Engine::d2a(direction.unwrap().unwrap()).unwrap()) {
                    x if x > self.ms_to_ticks(self.das) as usize || x == 1  => {
                        self.block.shift(direction.unwrap().unwrap());
                    },
                    _ => ()
                }
            }
        };

        // Handle rotations
        if self.controller.time(Action::RotateLeft) == 1 {
            self.block.rotate(Rotation::R270);
        }
        if self.controller.time(Action::RotateRight) == 1 {
            self.block.rotate(Rotation::R90);
        }

        // Handle hold
        match self.hold {
            Some(hold) => {
                // TODO: May need a temporary here depending on binding
                self.hold = Some(self.block);
                self.block = hold.reset();
            },
            None => {
                self.hold = Some(self.block);
                self.block = Block::new(&self.field, self.randomizer.next().unwrap(), Rotation::R0);
            }
        };

        // Handle hard drop
        if self.controller.time(Action::HardDrop) == 1 {
            self.block.shift_extend(Direction::Down);
            self.field.freeze(self.block);
            self.block = Block::new(&self.field, self.randomizer.next().unwrap(), Rotation::R0);
        }
    }

    fn update_gameover(&self) {
    }

    fn update_excellent(&self) {
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_engine() {
        let field = Field::new();
        let randomizer = BagRandomizer::new();
        let wallkick = SRS::new();
        let mut engine = Engine(&randomizer, &wallkick, &field);

        while engine.running {
            // Update keys - (Managed externally)

            // call update
            engine.update();

            // Field is in next state iteration
        }
    }
}
