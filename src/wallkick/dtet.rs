//! Implements the DTET wallkick.
//!
//! The DTET wallkick is a symmetric wallkick initially appearing
//! in the DTET tetris game.

use block::{Rotation, Block};
use field::Field;
use wallkick::Wallkick;

gen_wallkick!(DTET);

    static RIGHT_ROTATION: [(i32, i32); 6] = [
        (0, 0), (1, 0), (-1, 0), (1, 0), (1, 1), (-1, 1)
    ];

    static LEFT_ROTATION: [(i32, i32); 6] = [
        (0, 0), (-1, 0), (1, 0), (1, 0), (-1, 1), (1, 1)
    ];

    static NONE_ROTATION: [(i32, i32); 1] = [
        (0, 0)
    ];

impl Wallkick for DTET {
    #[allow(unused_variables)]
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        match r {
            Rotation::R90  => &RIGHT_ROTATION,
            Rotation::R270 => &LEFT_ROTATION,
            _ => &NONE_ROTATION
        }
    }
}
