//! Implements the TGM3 wallkick.
//!
//! This handles the special I and T cases, otherwise it will revert back to
//! traditional TGM wallkick behavior.
//!
//! TODO: How do we want to handle floorkick restriction limits? I'd argue for
//! an engine restriction nearly, determining the particular rotation type found
//! and counting it there. This would provide greater customisability at the
//! expense of more complicated engine logic.
//!
//! Alternatively, we manage it internally, however this would required caching of
//! blocks that have been floorkicked, and could prove difficult when attempting
//! to handle multi-blocks (i.e. doubles mode).
//!
//! Finally, could we just add more fields to each particular block? This would
//! remove problems managing the data, but adds extra complexity to a block
//! primitive. Need to explore other special cases and see how this behavior
//! best fits with these.
//!
//! These floorkick limits appear to be enforced by lock delay implicitly. Is
//! it event required to manage a counter?
//!
//! An option for disabling all floorkicks can easily be managed in the engine
//! seperate of this so it shouldn't factor in to the argument. Having a
//! single floorkick count can also be achieved, the only problem is differentiating
//! amongst different floorkick types?
//!
//! Could just have a floorkick count for each block type?

use block::{Rotation, Block, Id};
use field::Field;
use wallkick::{self, Wallkick};

gen_wallkick!(TGM3);

static NONE_ROTATION: [(i32, i32); 1] = [
    (0, 0)
];

static I_FLOORKICK_ROTATION: [(i32, i32); 3] = [
    (0, 0), (0, -1), (0, -2)
];

static I_ROTATION: [(i32, i32); 4] = [
    (0, 0), (1, 0), (2, 0), (-1, 0)
];

static T_FLOORKICK_ROTATION: [(i32, i32); 2] = [
    (0, 0), (0, -1)
];

impl Wallkick for TGM3 {
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        if block.id == Id::I {
            // Check if any field pieces exist beneath the I block. Wallkicks
            // are not allowed in mid-air.
            // This should check for vertical to horizontal and ignore?
            // TODO: Add floorkick limit of 1.
            if block.rs.data(block.id, block.r).iter().any(|&(x, y)| {
                        field.occupies((usize!(block.x + i32!(x)), usize!(block.y + i32!(y) + 1)))
                    }) {
                // Should attempt floorkick
                return &I_FLOORKICK_ROTATION;
            }
            // Check wallkicks. We cannot perform a floorkick with a wallkick,
            // (is this correct behavior?)
            else {
                return &I_ROTATION;
            }
        }
        // Check for T tetrimino stuck in a groove. This will kick upwards when
        // rotating from one of two rotations to the adjacent flatside.
        // TODO: Add floorkick limit of 2.
        else if block.id == Id::T {
            // The minimum piece offset of the block
            let (pxo, pyo) = block.rs.minp(block.id, block.r);

            // The absolute position of the blocks first piece; (y, x) ordered.
            let (apxo, apyo) = (usize!(block.x + i32!(pxo)), usize!(block.y + i32!(pyo)));

            // Height of the block
            let ph = block.rs.max(block.id, block.r).0 - block.rs.min(block.id, block.r).0;

            // Block data for the current piece
            let od = block.rs.data(block.id, block.r);

            // Filter out flatside rotation
            if !(ph == 1 && od.contains(&(apxo + 1, apyo))) {
                // Determine potentially grooved piece
                let (bxo, byo) = if ph == 1 { (apxo + 1, apyo + 1) } else { (apxo, apyo + 2) };

                // Ensure we don't get out of bounds with any proceeding calculations
                if !(byo >= field.height || byo + 1 >= field.width) {
                    // Check adjacent for stuck in groove
                    if field.occupies((bxo - 1, byo)) && field.occupies((bxo + 1, byo)) {
                        // Perform a floorkick!
                        return &T_FLOORKICK_ROTATION;
                    }
                    // No other wallkicks will work in groove so return none
                    else {
                        return &NONE_ROTATION;
                    }
                }
            }
        }

        // Fallback to traditional TGM specification
        wallkick::TGM::new().test(&block, &field, r)
    }
}
