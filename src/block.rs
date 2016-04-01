//! A block represents a single tetrimino and is always tied to a specific
//! field instance.

use Rotation;
use Direction;
use field::Field;

use std::{cmp, mem};
use collections::enum_set::CLike;

/// Rotation specifications. All the values are x, y coordinates that
/// extend down and right, with the origin being the upper-left corner
/// of the field.
///
/// Values can never be negative, as a blocks x, y value could always be
/// normalized to make these data points positive.
///
/// Currently this implements SRS rotations but we can theoretically have
/// more, so this should be generalized and hidden behind a trait at some
/// point.
static BLOCK_DATA: [[[(usize, usize); 4]; 4]; 7] = [
    [   // I-block
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],

    [   // T-block
        [(0, 1), (1, 0), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 0), (1, 1), (1, 2)],
    ],

    [   // L-block
        [(0, 1), (1, 1), (2, 0), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (0, 2), (1, 1), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],

    [   // J-block
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 0)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(0, 2), (1, 0), (1, 1), (1, 2)],
    ],

    [   // S-block
        [(0, 1), (1, 0), (1, 1), (2, 0)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(0, 2), (1, 1), (1, 2), (2, 1)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ],

    [   // Z-block
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(1, 1), (1, 2), (2, 0), (2, 1)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(0, 1), (0, 2), (1, 0), (1, 1)],
    ],

    [   // O-block
        [(1, 0), (1, 1), (2, 0), (2, 1)],
        [(1, 0), (1, 1), (2, 0), (2, 1)],
        [(1, 0), (1, 1), (2, 0), (2, 1)],
        [(1, 0), (1, 1), (2, 0), (2, 1)],
    ]
];

/// The type of a specific block.
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Type {
    I, T, L, J, S, Z, O, None
}

impl CLike for Type {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(v: usize) -> Type {
        assert!(v < 7);
        unsafe { mem::transmute(v) }
    }
}

impl Type {
    /// Returns all known types
    pub fn variants() -> Vec<Type> {
        vec![Type::I, Type::T, Type::L, Type::J, Type::S, Type::Z, Type::O]
    }
}


/// A block.
///
/// All coordinates are taken from the top-left of the field, increasing y
/// moves the block down, increasing x moves the block right.
///
/// This is done to preserve compatibility with how most GUI systems provide
/// their coordinates.
#[derive(Hash, Clone, Debug)]
pub struct Block {
    /// X-coordinate of the piece
    pub x:  i32,

    /// Y-coordinate of the piece
    pub y:  i32,

    /// Type of the block
    pub id: Type,

    /// Rotation state of the block
    pub r: Rotation,

    /// Block offset data
    pub data: &'static [(usize, usize)],
}

impl Block {
    /// Construct a new block from the specified input parameters.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::block::Block;
    ///
    /// let block = Block::new(tetrs::block::Type::I, tetrs::Rotation::R90);
    /// ```
    pub fn new(id: Type, r: Rotation) -> Block {
        Block { x: 4,
                y: 0,
                id: id,
                r: r,
                data: &BLOCK_DATA[id.to_usize()][r.to_usize()],
        }
    }

    /// Construct a block at the specific location on the field.
    ///
    /// If the location is invalid and is filled, then return None. This is
    /// not often used, but can be useful for specific cases.
    pub fn new_at(id: Type, r: Rotation, (x, y): (usize, usize)) -> Block {
        Block { x: x as i32,
                y: y as i32,
                id: id,
                r: r,
                data: &BLOCK_DATA[id.to_usize()][r.to_usize()],
        }
    }

    /// Consumes the current block returning a new block that has been
    /// `reset` (respawned).
    ///
    /// This is useful when implementing a hold function
    pub fn reset(self) -> Block {
        Block::new(self.id, Rotation::R0)
    }

    /// Returns a tuple with the minimum x, y values of the current block.
    pub fn leading(&self) -> (i32, i32) {
        self.data.iter()
                 .map(|&(x, y)| (x as i32, y as i32))
                 // TODO: Use an integer constant instead
                 .fold((100, 100), |(a, b), (x, y)| {
                     (cmp::min(a, x), cmp::min(b, y))
                 })
    }

    /// Returns a tuple with the maximum x, y values of the current block
    pub fn trailing(&self) -> (i32, i32) {
        self.data.iter()
                 .map(|&(x, y)| (x as i32, y as i32))
                 .fold((0, 0), |(a, b), (x, y)| {
                     (cmp::max(a, x), cmp::max(b, y))
                 })
    }


    /// Checks if the current block collides after applying the offset.
    fn collision_at(&self, field: &Field, (xo, yo): (i32, i32)) -> bool {
        self.data.iter()
                  .map(|&(dx, dy)| {
                      (self.x + dx as i32 + xo, self.y + dy as i32 + yo)
                  })
                  .any(|(x, y)| {
                      let maxf = self.trailing();
                      let minf = self.leading();

                      x - minf.0 < 0 || x + maxf.0 > field.width as i32 ||
                      y - minf.1 < 0 || y + maxf.1 > field.height as i32 ||
                      field.at((x as usize, y as usize)) != Type::None.to_usize()
                  })
    }

    /// Checks if the current block collides with anything on field.
    pub fn collision(&self, field: &Field) -> bool {
        self.collision_at(&field, (0, 0))
    }


    /// Shift the block by the specified coordinates.
    ///
    /// This does not check intermediate iterations so we can effectively
    /// teleport over blocks if we move far enough.
    fn shift_raw(&mut self, field: &Field, (x, y): (i32, i32)) -> bool {
        if !self.collision_at(&field, (x, y)) {
            self.x += x;
            self.y += y;
            true
        }
        else {
            false
        }
    }

    /// Shift the block one place by the specified direction.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::Block;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(tetrs::block::Type::Z, tetrs::Rotation::R0);
    /// block.shift(&field, tetrs::Direction::Left);
    /// ```
    pub fn shift(&mut self, field: &Field, direction: Direction) -> bool {
        let (x, y): (i32, i32) = match direction {
            Direction::Left =>  (-1, 0),
            Direction::Right => ( 1, 0),
            Direction::Down =>  ( 0, 1),
            _ => panic!("Found invalid direction: {:?}", direction)
        };

        self.shift_raw(&field, (x, y))
    }

    /// Repeatedly shift a block as far as we can until a collision occurs.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::Block;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(tetrs::block::Type::Z, tetrs::Rotation::R0);
    /// block.shift_extend(&field, tetrs::Direction::Left);
    /// ```
    pub fn shift_extend(&mut self, field: &Field, direction: Direction) {
        while self.shift(&field, direction) {}
    }


    /// Set the rotation state to the specified.
    fn rotation_raw(&mut self, rotation: Rotation) {
        self.r = rotation;
        self.data = &BLOCK_DATA[self.id.to_usize()][self.r.to_usize()];
    }

    /// Rotate the block by the specified amount and apply an offset.
    /// This is a helper function for calculating wallkick values easily.
    pub fn rotate_with_offset(&mut self, field: &Field, rotation: Rotation, (x, y): (i32, i32)) -> bool {
        let original_rotation = self.r;
        let new_rotation = match rotation {
            Rotation::R0   => self.r,
            Rotation::R90  => self.r.clockwise(),
            Rotation::R180 => self.r.clockwise().clockwise(),
            Rotation::R270 => self.r.anticlockwise()
        };

        self.rotation_raw(new_rotation);

        if self.shift_raw(&field, (x, y)) {
            true
        }
        else {
            self.rotation_raw(original_rotation);
            false
        }
    }

    /// Rotate the block by the specified amount.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::Block;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(tetrs::block::Type::Z, tetrs::Rotation::R0);
    /// block.rotate(&field, tetrs::Rotation::R90);
    /// ```
    pub fn rotate(&mut self, field: &Field, rotation: Rotation) -> bool {
        self.rotate_with_offset(&field, rotation, (0, 0))
    }

    /// Check if the block occupies the specified offset.
    pub fn at(&self, (a, b): (usize, usize)) -> bool {
        self.data.iter()
            .map(|&(x, y)| (self.x as usize + x, self.y as usize + y))
            .any(|(x, y)| a == x && b == y)
    }


    /// Return a new block that is this blocks ghost.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::Block;
    ///
    /// let field = Field::new();
    /// let block = Block::new(tetrs::block::Type::Z, tetrs::Rotation::R0);
    /// let ghost = block.ghost(&field);
    /// ```
    pub fn ghost(&self, field: &Field) -> Block {
        let mut ghost = self.clone();
        ghost.shift_extend(&field, Direction::Down);
        ghost
    }

    /// Return the block data for the specified type.
    pub fn data(id: Type, r: Rotation) -> &'static [(usize, usize)] {
        &BLOCK_DATA[id.to_usize()][r.to_usize()]
    }

    /// Return the block offsets for the specified type.
    pub fn offset(id: Type, r: Rotation) -> (usize, usize) {
        BLOCK_DATA[id.to_usize()][r.to_usize()].iter()
                 .fold((100, 100), |(a, b), &(x, y)| {
                     (cmp::min(a, x), cmp::min(b, y))
                 })
    }

    /// Return the offset to the first piece. This is slightly different
    /// from `offset` which itself returns the offsets of empty rows/cols.
    pub fn offset_to_first(id: Type, r: Rotation) -> (usize, usize) {
        BLOCK_DATA[id.to_usize()][r.to_usize()].iter()
                .fold((100, 100), |(a, b), &(x, y)| {
                    // We want the least-(x, y) such that y is minimized.
                    // This is subtly different from offset which allows the
                    // minimum of (x, y) from any multiple blocks.
                    if y < b || (y == b && x <= a) {
                        (x, y)
                    }
                    else {
                        (a, b)
                    }
                })
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use Rotation;
    use Direction;
    use field::Field;

    #[test]
    fn test_shift() {
        let field = Field::new();
        let mut block = Block::new(Type::Z, Rotation::R0);

        let x = block.x;
        block.shift(&field, Direction::Left);
        assert_eq!(x - 1, block.x);

        block.shift_extend(&field, Direction::Left);
        assert_eq!(0, block.x);
    }

    #[test]
    fn test_rotation() {
        let field = Field::new();
        let mut block = Block::new(Type::S, Rotation::R0);
        block.shift(&field, Direction::Down);
        block.shift(&field, Direction::Down);

        block.rotate(&field, Rotation::R90);
        assert_eq!(block.r, Rotation::R90);

        block.rotate(&field, Rotation::R90);
        assert_eq!(block.r, Rotation::R180);

        block.rotate(&field, Rotation::R270);
        assert_eq!(block.r, Rotation::R90);
    }

    #[test]
    fn test_offset_to_first1() {
        assert_eq!((1, 0), Block::offset_to_first(Type::T, Rotation::R0));
        assert_eq!((1, 0), Block::offset_to_first(Type::T, Rotation::R90));
        assert_eq!((0, 1), Block::offset_to_first(Type::T, Rotation::R180));
        assert_eq!((1, 0), Block::offset_to_first(Type::T, Rotation::R270));

        assert_eq!((2, 0), Block::offset_to_first(Type::I, Rotation::R90));
        assert_eq!((0, 0), Block::offset_to_first(Type::Z, Rotation::R0));
    }
}
