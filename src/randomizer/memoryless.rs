//! Implements a memoryless randomizer.

use std::collections::VecDeque;
use rand;
use rand::Rng;
use block;
use randomizer::Randomizer;

/// A generic memoryless randomizer.
///
/// This generates a completely arbitrary sequence of `block::Type`'s.
#[derive(Clone)]
pub struct MemorylessRandomizer {
    /// The lookahead buffer.
    lookahead: VecDeque<block::Type>,

    /// The rng used to generate random values
    rng: rand::ThreadRng
}

impl Randomizer for MemorylessRandomizer {
    fn new(lookahead: usize) -> MemorylessRandomizer {
        MemorylessRandomizer {
            lookahead: Vec::with_capacity(lookahead),
            rng: rand::thread_rng()
        }
    }
}

impl RandomizerPrivate for MemorylessRandomizer {
    fn next_block(&mut self) -> block::Type {
        self.rng.choose(block::Type::variants())
    }
}
