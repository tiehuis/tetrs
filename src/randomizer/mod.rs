//! Implements a randomizer.
//!
//! Randomizers currently only are required to implement the `Iterator`
//! trait. Preview pieces are managed by the caller, although this could be
//! something added as a trait.
//!
//! It is arguable whether `Randomizer` should imply `Iterator`. A `Randomizer`
//! is often only processed a single element at a time, and cannot be used in
//! an iterator fashion due to borrow rules.
//!
//! Also, all `Randomizer`'s should return infinite sequences so we can remove
//! the required `unwrap` on manual calls to `next`.

use block;

/// A randomizer must implement an iterator, plus a preview function which
/// returns a number of lookahead pieces.
pub trait Randomizer {
    /// Construct a new Randomizer object with the specified lookahead.
    ///
    /// `lookahead` determines the size of the lookahead array and allows the
    /// library to manage these pieces instead of the caller.
    //
    // This feels more suitable to not be part of the trait itself.
    fn new(lookahead: usize) -> Self;

    /// Return a vector containing the next `n` pieces that will be retrieved
    /// by the iterator.
    ///
    /// `n` must be < `lookahead` else a panic will be issued.
    fn preview(&mut self, lookahead: usize) -> Vec<block::Type>;

    /// Return the next block value in this sequence.
    ///
    /// All sequences should be infinite, and iterator use is limited so we use
    /// a custom function on this trait instead of implementing `Iterator`.
    fn next(&mut self) -> block::Type;
}

/// `next_block` should not be exposed as calling it can produce unexpected
/// results (it modifies the internal buffer).
///
/// Thus, this is hidden behind a private trait.
trait RandomizerPrivate : Randomizer {
    /// Generate the next block in sequence for this randomizer.
    ///
    /// This is not exposed as part of the API so is part of a private trait.
    fn next_block(&mut self) -> block::Type;
}

// This macro can be used to generate the `lookahead` and `next` functions for
// the given randomizer. These are generic across all randomizers but with the
// lack of inheritance we resort to this method of generation.
macro_rules! gen_rand {
    ($rdid:ident) => {
        impl Randomizer for $rdid {
            fn preview(&mut self, amount: usize) -> Vec<block::Type> {
                assert!(amount < self.lookahead.capacity());

                if self.lookahead.len() < amount {
                    let randvalue = self.next_block();
                    self.lookahead.push(randvalue);
                }

                self.lookahead.iter().cloned().take(amount).collect::<Vec<_>>()
            }

            fn next(&mut self) -> block::Type {
                if self.lookahead.is_empty() {
                    self.gen_block()
                }
                else {
                    self.lookahead.pop_front().unwrap()
                }
            }
        }
    }
}

pub use self::bag_randomizer::BagRandomizer;
mod bag_randomizer;
