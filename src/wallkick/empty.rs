//! Implements the empty wallkick.
//!
//! The empty wallkick only tests the trivial offset (0, 0) and if this
//! fails then the rotation will fail.

use block::{Rotation, Block};
use field::Field;
use wallkick::Wallkick;

gen_wallkick!(Empty);

impl Wallkick for Empty {
    #![allow(unused_variables)]
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        static NO_WALLKICK: [(i32, i32); 1] = [(0, 0)];
        &NO_WALLKICK
    }
}
