//! Implements the Gameboy randomizer

use std::collections::VecDeque;
use rand::{self, Rng};
use block::Type;
use randomizer::Randomizer;

gen_rand!(GameboyRandomizer);

/// A generic memoryless randomizer.
//
/// This generates a completely arbitrary sequence of `Type`'s.
#[derive(Clone)]
pub struct GameboyRandomizer {
    /// The lookahead buffer.
    lookahead: VecDeque<Type>,

    /// The rng used to generate random values
    rng: rand::ThreadRng,

    /// Last piece id
    prev: usize
}

impl GameboyRandomizer {
    /// Return a new `GameboyRandomizer` instance.
    pub fn new(lookahead: usize) -> GameboyRandomizer {
        let mut gb = GameboyRandomizer {
            lookahead: VecDeque::with_capacity(lookahead),
            rng: rand::thread_rng(),
            prev: 0
        };

        gb.prev = gb.rng.gen_range(0, Type::variants().len());
        gb
    }

    fn next_block(&mut self) -> Type {
        let variants = Type::variants();
        let roll = 6 * variants.len() - 3;
        self.prev += ((self.rng.gen_range(0, roll) / 5) + 1) % variants.len();
        variants[self.prev]
    }
}
