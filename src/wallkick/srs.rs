//! Implements the wallkicks for the SRS rotation system.

/// Wallkick test implementing the SRS rotation system. Most wallkick structs
/// do not have any data associated that is not of 'static lifetime.
pub struct SRS;

use Rotation;
use block::Block;
use block;
use wallkick::WallkickTest;

use collections::enum_set::CLike;

impl WallkickTest for SRS {
    /// Wallkick tests for the specified id and rotation.
    ///
    /// Wallkick test values are expected to lie in a static array currently.
    /// This may be changed in the future if something is more applicable.
    ///
    /// ## Examples
    /// ```ignore
    /// let block = Block::new(...);
    /// let wallkick = SRS::new(...);
    ///
    /// for (tx, ty) in wallkick.test(&block, rotation) {
    ///     if block.rotate_with_offset(rotation, (tx, ty)) {
    ///         break;
    ///     }
    /// }
    /// ```
    fn test(&self, block: &Block, r: Rotation) -> &'static [(i32, i32)] {
        match r {
            Rotation::R90 => {
                if block.id == block::Type::I {
                    &RIGHT_I[block.r.to_usize()][..SRS::count(block.id)]
                }
                else {
                    &RIGHT_JLSTZ[block.r.to_usize()][..SRS::count(block.id)]
                }
            },
            Rotation::R270 => {
                if block.id == block::Type::I {
                    &LEFT_I[block.r.to_usize()][..SRS::count(block.id)]
                }
                else {
                    &LEFT_JLSTZ[block.r.to_usize()][..SRS::count(block.id)]
                }
            },
            _ => panic!("Invalid wallkick test")
        }
    }
}

impl SRS {
    // Not all pieces have the same number of tests
    fn count(id: block::Type) -> usize {
        if id.to_usize() < 6 { 5 } else { 1 }
    }
}

// Wallkick data for all items.
static RIGHT_JLSTZ: [[(i32, i32); 5]; 4] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
];


static LEFT_JLSTZ: [[(i32, i32); 5]; 4] = [
    [(0, 0), (1, 0), (1, 1), ( 0, -2), ( 1, -2)],
    [(0, 0), (-1, 0), (-1, -1), ( 0, 2), (-1, 2)],
    [(0, 0), (-1, 0), (-1, 1), ( 0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
];


static RIGHT_I: [[(i32, i32); 5]; 4] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (2, -1)]
];


static LEFT_I: [[(i32, i32); 5]; 4] = [
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
];
