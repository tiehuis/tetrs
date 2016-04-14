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
        // (i.e. don't assume rotation 1 and 3 are 3-wide!)
        //
        // Due to differing rotations, we cannot perform simple calculations
        // assuming a bounding box. In some cases, we are effectively scanning
        // for a particular block configuration.
        if block.id == Id::L || block.id == Id::J || block.id == Id::T {
            // Calculate the width of the piece
            let (txo, tyo) = block.rs.max(block.id, block.r);
            let (lxo, lyo) = block.rs.min(block.id, block.r);
            let (pxo, pyo) = block.rs.minp(block.id, block.r);
            let (apxo, apyo) = (usize!(block.x + i32!(pxo)), usize!(block.y + i32!(pyo)));
            let od = block.rs.data(block.id, block.r);

            // Block is 3-wide
            if txo - lxo == 3 {
                // Determine where the middle column lies and check the
                // required location for each rotation.

                let mxo = lxo + 1;

                // Only need to check one square above center column
                if block.id == Id::T {
                    if field.occupies((mxo, lyo - 1)) {
                        return &NONE_ROTATION;
                    }
                }
                // We need to determine the specific rotation
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

        // Every other field state wallkicks are allowed

        match r {
            Rotation::R90 | Rotation::R270 => &ROTATION,
            _ => &NONE_ROTATION
        }
    }
}
