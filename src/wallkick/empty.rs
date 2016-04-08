//! Implements the empty wallkick

use block::{Rotation, Block};
use wallkick::Wallkick;

gen_wallkick!(Empty);

impl Wallkick for Empty {
    #![allow(unused_variables)]
    fn test(&self, block: &Block, r: Rotation) -> &'static [(i32, i32)] {
        static NO_WALLKICK: [(i32, i32); 1] = [(0, 0)];
        &NO_WALLKICK
    }
}
