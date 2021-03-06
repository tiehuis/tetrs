//! Implements a simple wallkick.
//!
//! The simple wallkick algorithm will only attempt one left and then right
//! movement if the rotation fails.

use block::{Rotation, Block};
use field::Field;
use wallkick::Wallkick;

gen_wallkick!(Simple);

impl Wallkick for Simple {
    #![allow(unused_variables)]
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        static SIMPLE_WALLKICK: [(i32, i32); 3] = [(0, 0), (1, 0), (-1, 0)];
        &SIMPLE_WALLKICK
    }
}
