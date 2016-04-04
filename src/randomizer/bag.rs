//! Implements a 7-element bag randomizer.

use std::collections::VecDeque;
use rand;
use rand::Rng;
use block::Type;
use randomizer::Randomizer;

gen_rand!(BagRandomizer);

/// A generic bag randomizer.
///
/// This randomizer generates sequences of all 7-blocks and shuffles them,
/// allowing a maximum distance between block sightings of 13.
///
/// ```ignore
/// use randomizer::{BagRandomizer, Randomizer};
///
/// // Generate a bag with a 15-piece lookahead
/// let bag = BagRandomizer::new(15);
//
/// let piece1 = bag.next();
/// let previews = bag.preview(4); // Get upcoming 4 pieces
/// let piece2 = bag.next();
/// ```
#[derive(Clone)]
pub struct BagRandomizer {
    /// The lookahead buffer.
    lookahead: VecDeque<Type>,

    /// The rng used to generate random values
    rng: rand::ThreadRng,

    /// The current index of the bag
    head: usize,

    /// The pieces in the bag
    data: [Type; 7],
}

impl BagRandomizer {
    /// Generate a new `BagRandomizer` instance.
    pub fn new(lookahead: usize) -> Self {
        let mut bag = BagRandomizer {
            lookahead: VecDeque::with_capacity(lookahead),
            rng: rand::thread_rng(),
            head: 0,
            data: [Type::None; 7],
        };

        // Pre-fill bag with all blocks and shuffle
        bag.data.clone_from_slice(Type::variants());
        bag.rng.shuffle(&mut bag.data[..]);
        bag
    }

    /// Generate the next block in the sequence
    fn next_block(&mut self) -> Type {
        let id = self.data[self.head];
        if self.head + 1 == self.data.len() {
            self.rng.shuffle(&mut self.data[..]);
            self.head = 0;
        }

        id
    }
}
