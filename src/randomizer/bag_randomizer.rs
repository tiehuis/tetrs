//! Implements a 7-element bag randomizer.
//!
//! Currently it can be confusing dealing with iterators and their clones etc
//! so there may be a slightly better way in working with these.

use rand;
use rand::Rng;

use block;
use randomizer::Randomizer;

/// A generic bag randomizer.
///
/// This randomizer allows a 7 piece lookahead and guarantees a maximum
/// distance between blocks of the same type to be 13 blocks apart.
///
/// Lookahead pieces are provided as an vector currently, but when abstract
/// return types are implemented this can be changed easily to an iterator.
///
/// ## Examples
/// ```ignore
/// let bag = BagRandomizer::new();
///
/// for piece_type in bag.preview().iter() {
///     // Do something with the block
/// }
///
/// ```
#[derive(Clone)]
pub struct BagRandomizer {
    head: usize,
    data: [block::Type; 14],
    rng: rand::ThreadRng
}

impl BagRandomizer {
    /// Return a new BagRandomizer instance.
    pub fn new() -> BagRandomizer {
        let mut bag = BagRandomizer {
            // FIXME: a map, cycle, take, reduce encounters the following:
            // the trait `core::clone::Clone` is not implemented for the type
            // `[closure@randomizer.rs:52:30: 52:60]` [E0277]
            //
            // This can probably be fixed, just not sure how right now. So we
            // fall back to an outside initialization.
            data: [block::Type::None; 14],
            head: 0,
            rng: rand::thread_rng()
        };

        for (i, v) in block::Type::variants().into_iter().cycle().take(14).enumerate() {
            bag.data[i] = v;
        }

        bag.shuffle(true);
        bag.shuffle(false);
        bag
    }

    /// Shuffle half the bag contents.
    ///
    /// ## Examples
    /// ```ignore
    /// let mut bag = BagRandomizer::new();
    ///
    /// bag.shuffle(true);  // Shuffles the front 7 elements
    ///
    /// bag.shuffle(false); // Shuffles the back 7 elements
    /// ```
    fn shuffle(&mut self, front: bool) {
        let offset = if front { 0 } else { 7 };
        self.rng.shuffle(&mut self.data[offset..(offset+7)]);
    }
}

impl Randomizer for BagRandomizer {
    fn preview(&self) -> Vec<block::Type> {
        let mut pieces = Vec::with_capacity(7);

        for i in 0..7 {
            pieces.push(self.data[(self.head + i) % 14])
        }

        pieces
    }
}

impl Iterator for BagRandomizer {
    type Item = block::Type;

    fn next(&mut self) -> Option<block::Type> {
        let id = self.data[self.head];
        self.head = (self.head + 1) % 14;

        // Copy required until MIR improvements
        let head_value = self.head;

        if self.head % 7 == 0 {
            // If the head is 7, we have just consumed elements 0-6 so we
            // can reshuffle, else it is 0, so we shuffle the back.
            self.shuffle(head_value == 7);
        }

        Some(id)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_cycle() {
        // Ensure values are seen as expected
        let bag = BagRandomizer::new();

        let mut seen = HashSet::new();

        for ty in bag.clone().take(7) {
            assert_eq!(seen.contains(&ty), false);
            seen.insert(ty);
        }
    }

    #[test]
    fn test_preview() {
        // Ensure previews are accurate when used with next values
    }
}
