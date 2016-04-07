//! Implements a memoryless randomizer.

use std::collections::VecDeque;
use rand::{self, Rng};
use block::Type;
use randomizer::Randomizer;

gen_rand!(MemorylessRandomizer);

/// A generic memoryless randomizer.
//
/// This generates a completely arbitrary sequence of `Type`'s.
#[derive(Clone)]
pub struct MemorylessRandomizer {
    /// The lookahead buffer.
    lookahead: VecDeque<Type>,

    /// The rng used to generate random values
    rng: rand::ThreadRng
}

impl MemorylessRandomizer {
    /// Return a new `MemorylessRandomizer` instance.
    pub fn new(lookahead: usize) -> MemorylessRandomizer {
        MemorylessRandomizer {
            lookahead: VecDeque::with_capacity(lookahead),
            rng: rand::thread_rng()
        }
    }

    fn next_block(&mut self) -> Type {
        *self.rng.choose(Type::variants()).unwrap()
    }
}
