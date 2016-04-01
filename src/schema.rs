//! Methods for converting to and from a textual field representation.
//!
//! This is tremendously useful for writing complex tests.
//!
//! Examples:
//!
//! ```ignore
//! use tetrs::schema::Schema;
//!
//! let (field, mut block) = Schema::from_string("
//!     |          |
//!     |  #       |
//!     | # @@     |
//!     |## @@     |
//!     ------------
//! ").to_state();
//!
//! block.rotate(&field, tetrs::Rotation::R90);
//!
//! assert_eq!(Schema::from_state(&field, &block), Schema::from_string("
//!     |          |
//!     |  #       |
//!     | # @@     |
//!     |#  @@     |
//!     ------------
//! "));
//! ```
//!
//! This can also potentially be useful for specifying initial fields of play.
//! This requires slightly more work in combining with other field
//! initialization parameters.

use Rotation;
use field::Field;
use block;
use block::Block;

use std::{fmt, iter};
use std::cmp::PartialEq;
use collections::enum_set::CLike;
use itertools::Itertools;

/// A schema is a simple 2d textual representation of a field and a block.
/// It allows conversion from a string, and also from a `(&Field, &Block)` and
/// bridges the gap between these two components.
///
/// This is stored in `(y, x)` index order to make some operations easier. This
/// is only an internal detail, but care needs to be taken when interfacing
/// with any other components (which are mostly `(x, y)` indexed).
#[derive(Clone, Debug)]
pub struct Schema {
    /// The current field data. This is a fixed square matrix.
    ///
    /// **This is stored in column-major order (i.e. (y, x) indexed)**
    data: Vec<Vec<char>>,

    /// The current width of the schema
    width: usize,

    /// The current height of the schema
    height: usize,
}

impl Schema {
    /// Construct a schema representation from an game primitives.
    pub fn from_state(field: &Field, block: &Block) -> Schema {
        let mut grid = vec![vec![' '; field.width]; field.height];
        let mut failure = false;

        // Problem constructing schema here
        for x in 0..field.width {
            for y in 0..field.height {
                grid[y][x] = match (field.set((x, y)), block.at((x, y))) {
                    (true, true) => {
                        failure = true;
                        'X'
                    },
                    (true, false) => '#',
                    (false, true) => '@',
                    _ => ' ',
                };
            }
        }

        // borrowck limitations
        let grid_width = grid[0].len();
        let grid_height = grid.len();

        let schema = Schema {
            data: grid,
            height: grid_height,
            // Assume height > 1
            width: grid_width
        };

        if failure {
            panic!("Collision in field and block: {}", schema);
        } else {
            schema
        }
    }

    /// Construct a schema representation from an input string.
    ///
    /// An input string is considered as line-seperated, with the field lying
    /// between pairs of `|` characters. Leading and trailing whitespace is
    /// ignored so different strings may produce the same schema.
    ///
    /// ## Examples
    /// ```ignore
    /// let schema1 = Schema::from_string("
    ///     |         |
    ///     |    #    |
    ///     -----------
    /// ");
    ///
    /// let schema2 = Schema::from_string("|          |
    ///    |    #     |
    ///  ------------");
    ///
    ///  assert_eq!(schema1, schema2); // True
    /// ```
    pub fn from_string(field: &str) -> Schema {
        let grid = field.split("\n")
                        .map(|s| {
                            s.trim()
                             .chars()
                             .filter(|&x| x != '\n' && x != '|' && x != '-')
                             .collect_vec()
                        })
                        .filter(|x| x.len() != 0)
                        .collect_vec();

        assert!(grid.len() != 0, "empty input");
        assert!(1 == grid.iter().map(|x| x.len()).dedup().count(), "uneven row lengths");

        // borrowck limitations
        let grid_width = grid[0].len();
        let grid_height = grid.len();

        Schema {
            data: grid,
            width: grid_width,
            height: grid_height
        }
    }

    /// Constuct state objects from a given schema. This is slightly finicky
    /// and there are a few cases to consider.
    ///
    /// It is possible that the schema itself is not well-formed and there
    /// is no suitable state to represent it, in which case we panic.
    ///
    /// It is up to the caller to ensure input validity. Since this is
    /// usually always used with static strings it is straight-forward enough
    /// in most cases.
    ///
    /// ## Examples
    /// ```ignore
    /// let schema1 = Schema::from_string("
    ///     |         |
    ///     |    @    |
    ///     -----------
    /// ");
    /// let (mut field, mut block) = schema1.to_state(); // Panic!
    ///
    /// let schema2 = Schema::from_string("
    ///     |    @    |
    ///     |   @@@   |
    ///     -----------
    /// ");
    /// let (mut field, mut block) = schema2.to_state(); // Okay
    /// ```
    pub fn to_state(&self) -> (Field, Block) {
        let mut schema = self.clone();

        // Currently cannot return a tuple-structure due to invalid lifetime
        // even though the lifetime theoretically has same scope.
        let mut field = Field::new();
        let mut block = None;

        // We must iterate over rows before columns
        for (y, x) in iproduct!(0..schema.height, 0..schema.width) {
            match schema.data[y][x] {
                '@' => {
                    block = Some(schema.match_block(&field, (x, y)));
                },
                '#' => {
                    field.data[x][y] = block::Type::None.to_usize();
                },
                ' ' => {
                    ()
                },
                _ => {
                    panic!("Encountered unknown character: \n{}", self);
                }
            }
        }

        // Testing with no block is pointless
        assert!(block.is_some());
        (field, block.unwrap())
    }

    // Return true if the specified x, y point is in the schema bounds and is
    // a block.
    fn is_block(&self, (x, y): (usize, usize)) -> bool {
        if x >= self.width || y >= self.height {
            false
        }
        else {
            if self.data[y][x] == '@' { true } else { false }
        }
    }

    // Determine which block block the current piece is part of.
    //
    // This assumes that we are traversing top -> bottom, left -> right.
    // If no block can be matched then we panic.
    //
    // This performs a bruteforce over all known blocks. Due to various
    // rotation ambiguities, we always return a block with the lowest
    // matching rotation in case of duplicates.
    //
    // ## Examples
    // ```ignore
    // let input = "
    // @@
    //  @@
    // ";
    //
    // // Matching block will always be rotation 0, and never rotation 2.
    // ```
    //
    // If it is required for exact rotations, then we could add support for
    // rotation specification in the input string, but this adds complexity
    // and more rules which are not needed currently.
    fn match_block(&mut self, field: &Field, (x, y): (usize, usize)) -> Block {
        for (&ty, &ro) in iproduct!(block::Type::variants().iter(), Rotation::variants().iter()) {
            let data = Block::data(ty, ro);
            let (xo, yo) = Block::offset_to_first(ty, ro);

            if x < xo || y < yo {
                continue;
            }

            let (ox, oy) = (x - xo, y - yo);

            if data.iter().all(|&(a, b)| self.is_block((ox + a, oy + b))) {
                data.iter().foreach(|&(a, b)| {
                    self.data[oy + b][ox + a] = ' ';
                });

                // (xo, yo) are not normalized based on the field so the block
                // needs to be adjusted.
                let block = Block::new_at(ty, ro, (ox, field.height - oy - 1));
                assert!(!block.collision(&field));
                return block;
            }
        }

        panic!("Failed to match any block: \n{}", self);
    }

    /// Construct a visual representation from a given schema.
    ///
    /// All output schema have their leading rows truncated for brevity.
    pub fn to_string(&self) -> String {
        "|".chars()
           .chain(self.data.iter()
                      .map(|x| x.clone().into_iter().collect::<String>())
                      .join("|\n|")
                      .chars())
           .chain("|\n".chars()
                       .chain(iter::repeat('-').take(self.width + 2)))
           .collect()
    }


    // Truncate a given schema to its simplest form.
    //
    // This removes all leading empty rows.
    fn truncate(&self) -> Schema {
        let mut schema = self.clone();
        let empty = iter::repeat(' ').take(self.width).collect_vec();

        schema.data.retain(|x| x.as_slice() != empty.as_slice());
        schema.height = schema.data.len();
        schema
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        // We can use an iterator here?
        if self.width == other.width {
            self.truncate().data.as_slice() == other.truncate().data.as_slice()
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction;

    #[test]
    fn test_from_string1() {
        let _schema = Schema::from_string("
                |          |
                |  # @     |
                | ##@@#    |
                |##  @#    |
                ------------
            ");
    }

    #[test]
    #[should_panic(expected = "uneven row lengths")]
    fn test_from_string2() {
        let _schema = Schema::from_string("
                |          |
                | # @     |
                | ##@@#    |
                |##  @#    |
                ------------
            ");
    }

    #[test]
    #[should_panic(expected = "empty input")]
    fn test_from_string3() {
        let _schema = Schema::from_string("
                        |
                      ");
    }

    #[test]
    fn test_from_state() {
        let schema = Schema::from_string("
                |          |
                |  @       |
                | @@@      |
                ------------
            ");

        let (field, mut block) = schema.to_state();
        block.shift(&field, Direction::Left);

        assert_eq!(Schema::from_state(&field, &block),
                   Schema::from_string("
                       |          |
                       | @        |
                       |@@@       |
                       ------------
                   "));
    }
}
