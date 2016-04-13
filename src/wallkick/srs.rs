//! Implements the wallkicks for the SRS rotation system.
//!
//! The SRS (Super Rotation System) wallkick is the current defacto standard
//! of wallkicks. The algorithm in question is slightly complicated and has
//! different rules depending on the block type.

use block::{self, Rotation, Block};
use field::Field;
use wallkick::Wallkick;
use collections::enum_set::CLike;

gen_wallkick!(SRS);

impl Wallkick for SRS {
    #[allow(unused_variables)]
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        // O block does not have any special wallkick data.
        if block.id == block::Id::O {
            return &RIGHT_JLSTZ[0][..1]
        }

        match r {
            Rotation::R90 => {
                if block.id == block::Id::I {
                    &RIGHT_I[block.r.to_usize()]
                }
                else {
                    &RIGHT_JLSTZ[block.r.to_usize()]
                }
            },
            Rotation::R270 => {
                if block.id == block::Id::I {
                    &LEFT_I[block.r.to_usize()]
                }
                else {
                    &LEFT_JLSTZ[block.r.to_usize()]
                }
            },
            _ => panic!("Invalid wallkick test")
        }
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
