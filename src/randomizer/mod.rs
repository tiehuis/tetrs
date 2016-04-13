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

use block::Id;

/// A randomizer must implement an iterator, plus a preview function which
/// returns a number of lookahead pieces.
pub trait Randomizer {
    /// Return a vector containing the next `n` pieces that will be retrieved
    /// by the iterator.
    ///
    /// `n` must be < `lookahead` else a panic will be issued.
    fn preview(&mut self, lookahead: usize) -> Vec<Id>;

    /// Return the next block value in this sequence.
    ///
    /// All sequences should be infinite, and iterator use is limited so we use
    /// a custom function on this trait instead of implementing `Iterator`.
    fn next(&mut self) -> Id;
}

// This macro can be used to generate the `lookahead` and `next` functions for
// the given randomizer. These are generic across all randomizers but with the
// lack of inheritance we resort to this method of generation.
macro_rules! gen_rand {
    ($id:ident) => {
        impl Randomizer for $id {
            fn preview(&mut self, amount: usize) -> Vec<Id> {
                assert!(amount <= self.lookahead.capacity());

                if self.lookahead.len() < amount {
                    let randvalue = self.next_block();
                    self.lookahead.push_back(randvalue);
                }

                self.lookahead.iter().cloned().take(amount).collect::<Vec<_>>()
            }

            fn next(&mut self) -> Id {
                if self.lookahead.is_empty() {
                    self.next_block()
                }
                else {
                    // Always succeeds
                    self.lookahead.pop_front().unwrap()
                }
            }
        }
    }
}

pub use self::bag::BagRandomizer;
pub use self::memoryless::MemorylessRandomizer;
pub use self::gameboy::GameboyRandomizer;
pub use self::tgm1::TGM1Randomizer;
pub use self::tgm2::TGM2Randomizer;

mod bag;
mod memoryless;
mod gameboy;
mod tgm1;
mod tgm2;

/// Factory function for generating randomizers.
///
/// # Names
///  - `bag`
///
/// # Panics
///
/// `new` will panic if the input string is not one of the strings present in
/// `Names`.
pub fn new(name: &str, lookahead: usize) -> BagRandomizer {
    match name {
        "bag" => BagRandomizer::new(lookahead),
        _ => panic!("unknown randomizer name")
    }
}
