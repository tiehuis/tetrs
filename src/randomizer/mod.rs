//! Implements a randomizer.

use block;

/// This trait must be implemented by all randomizers. This is in effect an
/// iterator with the specified type.
///
/// All randomizer do not actually store blocks, but instead block::Type
/// values which are used to construct blocks.
/// This is done so that the randomizer does not require extra constructor
/// knowledge in how to create a block itself (i.e. a field etc);
pub trait Randomizer : Iterator {
    /// Preview returns a list of upcoming blocks. This can be any length,
    /// including 0 length in the case of no previews.
    fn preview(&self) -> Vec<block::Type>;
}

pub mod bag_randomizer;
