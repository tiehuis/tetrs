//! Contains helper methods which expose more specific functions for
//! particular primitives.

use block::{Rotation, Block};
use field::Field;
use wallkick::Wallkick;

/// Helper traits for the `Block` struct.
pub trait BlockHelper {
    /// Rotate a `Block` using the specified `Wallkick` class.
    ///
    /// This iterates over all tests for the specified wallkick and attempts
    /// rotations until one is succcessful.
    fn rotate_with_wallkick(&mut self, field: &Field, wallkick: &Wallkick, rotation: Rotation) -> bool;
}

impl BlockHelper for Block {
    fn rotate_with_wallkick(&mut self, field: &Field, wallkick: &Wallkick, rotation: Rotation) -> bool {
        for &(x, y) in wallkick.test(&self, rotation) {
            if self.rotate_at_offset(&field, rotation, (x, y)) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::import::*;

    #[test]
    fn test_rotate_with_wallkick() {
        let field = Field::new();
        let mut block = Block::new(block::Id::S, &field);
        let wk = wallkick::SRS::new();

        block.shift(&field, Direction::Down);

        // Non-reference `wk` is slightly annoying.
        block.rotate_with_wallkick(&field, wk, Rotation::R90);
    }
}
