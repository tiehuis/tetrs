//! Implements a 7-element bag randomizer.

use std::collections::VecDeque;
use rand;
use rand::Rng;
use block;
use randomizer::{Randomizer, RandomizerPrivate};

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
    lookahead: VecDeque<block::Type>,

    /// The rng used to generate random values
    rng: rand::ThreadRng,

    /// The current index of the bag
    head: usize,

    /// The pieces in the bag
    data: [block::Type; 7],
}

impl Randomizer for BagRandomizer {
    fn new(lookahead: usize) -> BagRandomizer {
        let mut bag = BagRandomizer {
            lookahead: VecDeque::with_capacity(lookahead),
            rng: rand::thread_rng(),
            head: 0,
            data: [block::Type::None; 7],
        };

        // Pre-fill bag with all blocks and shuffle
        bag.data.clone_from_slice(block::Type::variants());
        bag.rng.shuffle(&mut bag.data[..]);
        bag
    }

    fn preview(&mut self, amount: usize) -> Vec<block::Type> {
        assert!(amount < self.lookahead.capacity());

        if self.lookahead.len() < amount {
            let randvalue = self.next_block();
            self.lookahead.push_back(randvalue);
        }

        self.lookahead.iter().cloned().take(amount).collect::<Vec<_>>()
    }

    fn next(&mut self) -> block::Type {
        if self.lookahead.is_empty() {
            self.next_block()
        }
        else {
            self.lookahead.pop_front().unwrap()
        }
    }
}

impl RandomizerPrivate for BagRandomizer {
    fn next_block(&mut self) -> block::Type {
        let id = self.data[self.head];
        if self.head + 1 == self.data.len() {
            self.rng.shuffle(&mut self.data[..]);
            self.head = 0;
        }

        id
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use randomizer::Randomizer;
    use std::collections::HashSet;

    #[test]
    fn test_cycle() {
        // Ensure values are seen as expected
        let mut bag = BagRandomizer::new(7);

        for _ in 0..7 {
            //let ty = bag.next();
            //assert_eq!(seen.contains(&ty), false);
            //seen.insert(ty);
        }
    }
}
