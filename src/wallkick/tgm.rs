//! Implements the TGM1/TGM2 wallkicks.

use block::{Rotation, Block, Id};
use field::Field;
use wallkick::Wallkick;

gen_wallkick!(TGM);

static ROTATION: [(i32, i32); 3] = [
    (0, 0), (1, 0), (-1, 0)
];

static NONE_ROTATION: [(i32, i32); 1] = [
    (0, 0)
];

impl Wallkick for TGM {
    #[allow(unused_variables)]
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        if block.id == Id::I {
            return &NONE_ROTATION;
        }

        // Check 3-wide block orientations and do not kick. Rotations may
        // differ slightly so we test the orientations in a generic way.
        // (i.e. don't assume rotation 1 and 3 are 3-wide!, also, don't assume
        // that they exist at x offset 0).
        if block.id == Id::L || block.id == Id::J || block.id == Id::T {
            // Stores the maximum x offset of the block
            let (txo, _) = block.rs.max(block.id, block.r);

            // Stores the least offsets of the block
            let (lxo, lyo) = block.rs.min(block.id, block.r);

            // Stores the offset to the first block piece; (y, x) ordered.
            let (pxo, pyo) = block.rs.minp(block.id, block.r);

            // The absolute position of the blocks first piece; (y, x) ordered.
            let (apxo, apyo) = (usize!(block.x + i32!(pxo)), usize!(block.y + i32!(pyo)));

            // Block piece data
            let od = block.rs.data(block.id, block.r);

            // Block must be 3-wide else ignore. We are dealing with offsets, so
            // we have to adjust by 1. (i.e) (0, 0) -> (0, 2) is height 3 (inclusive).
            if txo - lxo == 2 {
                // Middle column x position
                let mxo = lxo + 1;

                if block.id == Id::T {
                    if field.occupies((mxo, lyo - 1)) {
                        return &NONE_ROTATION;
                    }
                }
                else if block.id == Id::L {
                    // Non-flatside orientation
                    if od.contains(&(pxo + 1, pyo)) {
                        if !(field.occupies((apxo + 1, apyo + 1)) && field.occupies((apxo + 2, apyo - 1))) {
                            if field.occupies((mxo, apyo + 1)) || field.occupies((mxo, apyo - 1)) {
                                return &NONE_ROTATION;
                            }
                        }
                    }
                    // Flatside orientation
                    else {
                        if !(field.occupies((apxo + 1, apyo)) && field.occupies((apxo + 2, apyo - 1))) {
                            if field.occupies((mxo, apyo)) || field.occupies((mxo, apyo - 1)) {
                                return &NONE_ROTATION;
                            }
                        }
                    }
                }
                else {
                    // Non-flatside orientation
                    if od.contains(&(pxo + 1, pyo)) {
                        if !(field.occupies((apxo, apyo - 1)) && field.occupies((apxo + 1, apyo + 1))) {
                            if field.occupies((mxo, apyo - 1)) || field.occupies((mxo, apyo + 1)) {
                                return &NONE_ROTATION;
                            }
                        }
                    }
                    else {
                        if !(field.occupies((apxo - 1, apyo)) && field.occupies((apxo - 2, apyo - 1))) {
                            if field.occupies((mxo, apyo)) || field.occupies((mxo, apyo - 1)) {
                                return &NONE_ROTATION;
                            }
                        }
                    }
                }
            }
        }

        match r {
            Rotation::R0 => &NONE_ROTATION,
            Rotation::R90 | Rotation::R270 => &ROTATION,

            // If 180-degree spins are enabled, perform the standard wallkick
            // with no intermediate results. The special cases are still
            // filtered. This may be revised at some stage.
            Rotation::R180 => &ROTATION
        }
    }
}
