//! Implements a 7-element bag randomizer.

use std::collections::VecDeque;
use rand::{self, Rng};
use block::Id;
use randomizer::Randomizer;

gen_rand!(BagRandomizer);

/// A generic bag randomizer.
///
/// This randomizer generates sequences of all 7-blocks and shuffles them,
/// allowing a maximum distance between block sightings of 13.
///
/// ```
/// use tetrs::import::*;
///
/// // Generate a BagRandomizer using the factory function
/// let mut bag = randomizer::new("bag", 15);
///
/// // Generate a BagRandomizer directly
/// let bag2 = randomizer::BagRandomizer::new(15);
//
/// let piece1 = bag.next();
/// let previews = bag.preview(4); // Get upcoming 4 pieces
/// let piece2 = bag.next();
/// ```
#[derive(Clone)]
pub struct BagRandomizer {
    /// The lookahead buffer.
    lookahead: VecDeque<Id>,

    /// The rng used to generate random values
    rng: rand::ThreadRng,

    /// The current index of the bag
    head: usize,

    /// The pieces in the bag
    data: [Id; 7],
}

impl BagRandomizer {
    /// Generate a new `BagRandomizer` instance.
    pub fn new(lookahead: usize) -> Self {
        let mut bag = BagRandomizer {
            lookahead: VecDeque::with_capacity(lookahead),
            rng: rand::thread_rng(),
            head: 0,
            data: [Id::None; 7],
        };

        // Pre-fill bag with all blocks and shuffle
        bag.data.clone_from_slice(Id::variants());
        bag.rng.shuffle(&mut bag.data[..]);
        bag
    }

    /// Generate the next block in the sequence
    fn next_block(&mut self) -> Id {
        let id = self.data[self.head];

        self.head += 1;
        if self.head == self.data.len() {
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

    macro_rules! seq_test {
        ($r:ident) => {
            {
                let mut seen = Vec::new();
                for _ in 0..7 {
                    let piece = $r.next();
                    assert!(!seen.contains(&piece));
                    seen.push(piece);
                }
            }
        }
    }

    #[test]
    fn test_sequence() {
        let mut randomizer = BagRandomizer::new(7);

        seq_test!(randomizer);
        seq_test!(randomizer);
        seq_test!(randomizer);
    }
}
