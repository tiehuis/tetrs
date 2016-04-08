//! Implements a simple wallkick

use block::{Rotation, Block};
use wallkick::Wallkick;

gen_wallkick!(Simple);

impl Wallkick for Simple {
    #![allow(unused_variables)]
    fn test(&self, block: &Block, r: Rotation) -> &'static [(i32, i32)] {
        static SIMPLE_WALLKICK: [(i32, i32); 3] = [(0, 0), (1, 0), (-1, 0)];
        &SIMPLE_WALLKICK
    }
}
