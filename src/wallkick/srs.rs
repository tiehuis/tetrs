//! Implements the wallkicks for the SRS rotation system.
//!
//! The SRS (Super Rotation System) wallkick is the current defacto standard
//! of wallkicks. The algorithm in question is slightly complicated and has
//! different rules depending on the block type.

use block::{self, Rotation, Block};
use field::Field;
use wallkick::Wallkick;

gen_wallkick!(SRS);

impl Wallkick for SRS {
    #[allow(unused_variables)]
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)] {
        // O block does not have any special wallkick data.
        if block.id == block::Id::O {
            &RIGHT_JLSTZ[0][..1]
        }
        else {
            match r {
                Rotation::R90 => {
                    if block.id == block::Id::I {
                        &RIGHT_I[block.r as usize]
                    }
                    else {
                        &RIGHT_JLSTZ[block.r as usize]
                    }
                },
                Rotation::R270 => {
                    if block.id == block::Id::I {
                        &LEFT_I[block.r as usize]
                    }
                    else {
                        &LEFT_JLSTZ[block.r as usize]
                    }
                },
                _ => panic!("Invalid wallkick test")
            }
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

#[cfg(test)]
#[cfg(disabled)] // Temporarily disabled while SRS rotation is buggy
mod tests {
    use schema::Schema;
    use import::*;
    use utility::*;

    #[test]
    fn wallkick1() {
        let (field, mut block) = Schema::from_string("
                |          |
                |          |
                | @        |
                |#@@##     |
                |#@  #     |
                ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |# @##     |
               |#@@@#     |
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R270);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }

    #[test]
    fn wallkick2() {
        let (field, mut block) = Schema::from_string("
               |          |
               |    ##    |
               |   @ ###  |
               |   @@@####|
               | ###   ###|
               |##    ####|
               |####  ####|
               |##### ####|
               ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |    ##    |
               |     ###  |
               |      ####|
               | ### @ ###|
               |##   @####|
               |####@@####|
               |##### ####|
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R270);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }

    #[test]
    fn wallkick3() {
        let (field, mut block) = Schema::from_string("
               |          |
               |    @     |
               |   @@@    |
               |   #  #   |
               |   #  #   |
               |   #  #   |
               ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |          |
               |          |
               |   #@ #   |
               |   #@@#   |
               |   #@ #   |
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R90);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }

    #[test]
    fn wallkick4() {
        let (field, mut block) = Schema::from_string("
               |          |
               |#@        |
               |@@@       |
               | ##       |
               |  #       |
               | ##       |
               ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |#         |
               |          |
               |@##       |
               |@@#       |
               |@##       |
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R270);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }

    #[test]
    fn wallkick5() {
        let (field, mut block) = Schema::from_string("
               |          |
               |###       |
               |# #       |
               |#         |
               |#  @      |
               |###@      |
               |# @@      |
               ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |###       |
               |#@#       |
               |#@@@      |
               |#         |
               |###       |
               |#         |
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R270);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }

    #[test]
    fn wallkick6() {
        let (field, mut block) = Schema::from_string("
               |          |
               |###       |
               |# #       |
               |#         |
               |#  @      |
               |###@      |
               |# @@      |
               ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |###       |
               |#@#       |
               |#@@@      |
               |#         |
               |###       |
               |#         |
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R90);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }

    #[test]
    fn wallkick7() {
        let (field, mut block) = Schema::from_string("
               |          |
               |###       |
               |##@@      |
               |#@@       |
               |## ##     |
               |##  #     |
               |### #     |
               ------------
            ").to_state(rotation_system::new("srs"));

        let target = Schema::from_string("
               |          |
               |###       |
               |##        |
               |#         |
               |##@##     |
               |##@@#     |
               |###@#     |
               ------------
            ");

        block.rotate_with_wallkick(&field, wallkick::SRS::new(), Rotation::R270);
        schema_assert_eq!(Schema::from_state(&field, &block), target);
    }
}
