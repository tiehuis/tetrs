//! Implements a wallkick test
//!
//! A wallkick test returns the number of rotation offsets that should be
//! attempted on rotation failure.
//!
//! It should be implemented as an iterator (currently a 'static vector)
//! which returns (i32, i32) tuples.

use Rotation;
use block::Block;

/// Trait which specifies what wallkick tests must implement. Every wallkick
/// test must implement an iterator with offsets of type (i32, i32).
pub trait WallkickTest {
    /// Returns a set of wallkick tests for the specified block and rotation
    fn test(&self, block: &Block, r: Rotation) -> &'static [(i32, i32)];
}

pub mod srs;
