//! A single tetrimino.
//!
//! A `Block` consists of a `Type`, `Location` and `Rotation`. It does not
//! have to be considered part of a `Field`, however most movement functions
//! take a `Field` with which movement is verified against.
//!
//! ```ignore
//! use tetrs::{field, block, Rotation, Direction};
//!
//! let field = field::new();
//!
//! // Can spawn a block on a field directly...
//! let block = block::new().on_field(&field);
//!
//! // ...or independently
//! let block = block::new().position((5, 10));
//! ```

use Rotation;
use Direction;
use field::Field;
use rotation;
use rotation::RotationSystem;

use std::mem;
use collections::enum_set::CLike;

/// The identifier for a particular block.
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq, Eq)]
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
    /// Returns all `non-None` `Type` variants.
    ///
    /// ```
    /// let types = tetrs::block::Type::variants();
    /// ```
    pub fn variants() -> Vec<Type> {
        vec![Type::I, Type::T, Type::L, Type::J, Type::S, Type::Z, Type::O]
    }
}

/// A struct representing a single tetrimino.
///
/// Blocks are defined by an `(x, y)` coordinate pair, a `Rotation`, `Type`,
/// and an accompanying set of offset data which specifies exactly which
/// pieces in the field this block occupies.
///
/// Internally, `(x, y)` coordinates are taken with their origin from the top
/// right of the grid, to preserve some compatibility with GUI systems. All
/// internal block calculations follow this as well:
///
/// ```ignore
/// (0, 0)-----------------(10, 0)
///   |                       |
///   |                       |
///   |        .(4, 12)       |
///   |                       |
///   |                       |
///   |                       |
/// (0, 25)----------------(0, 25)
/// ```
///
/// Internally, the `(x, y)` and `data` fields look like the following:
///
/// ```ignore
///   (x, y) = (4, 4)
///     data = [(1, 0), (0, 1), (1, 1), (2, 1)]
/// ```
///
/// When calculating a block's position, the data offsets are added to the
/// base coordinates to produce the block. The previous block is a `Type::T`,
/// with `Rotation::R0`, as the final data represented is:
///
/// ```ignore
///  block = [(5, 4), (4, 5), (5, 5), (6, 5)]
/// ```
///
/// It is important to note that offsets are all non-negative, and as such the
/// coordinate effectively partitions the space where the block can reside.
/// Further, a block could have many different internal representations which
/// appear equal, by adjusting the `(x, y)` coordinates and `data` in
/// conjunction.
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

    /// Rotation system used internally
    pub rs: rotation::SRS
}

/// Traits for building a block.
///
/// This is used to override given default block values to more specific
/// values if required.
///
/// These should only be used on construction of a new block with
/// `block::new()`. This is not enforced, but limited checking is done
/// internally and the results may not be as one would expect.
///
/// No collision testing is done on the resulting block, so it is up to the
/// caller to perform this.
///
/// ## Examples
/// ```ignore
/// use tetrs::block::{Type, Block, BlockBuilder};
///
/// let block = Block::new(Type::I)
///                   .rotation(tetrs::Rotation::R270);
///                   .position((5, 10));
/// ```
///
/// See `block::new()` for what default values are used.
pub trait BlockBuilder {
    /// Alter the initial position of the block.
    fn position(self, position: (usize, usize)) -> Block;

    /// Alter the initial rotation of the block.
    fn rotation(self, rotation: Rotation) -> Block;

    /// Alter the initial position of the block, setting it to the spawn
    /// position as specified by `field`.
    fn on_field(self, field: &Field) -> Block;
}

impl BlockBuilder for Block {
    fn position(mut self, position: (usize, usize)) -> Block {
        self.x = position.0 as i32;
        self.y = position.1 as i32;
        self
    }

    fn rotation(mut self, rotation: Rotation) -> Block {
        self.r = rotation;
        self
    }

    fn on_field(mut self, field: &Field) -> Block {
        self.x = field.spawn.0 as i32;
        self.y = field.spawn.1 as i32;
        self
    }
}


impl Block {
    /// Construct a new default `Block` and return it.
    ///
    /// The only required value for a block is `Type`, otherwise the following
    /// default values are used:
    ///
    /// ### (`x`, `y`)
    /// The default spawn value is `(4, 0)`. This is a common spawn position
    /// for fields of standard size.
    ///
    /// ### Rotation
    /// The default rotation is `Rotation::R0`.
    ///
    /// ---
    ///
    /// If values are required to be overridden, look at the `BlockBuilder`
    /// trait implementation.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::block::Block;
    ///
    /// let block = Block::new(tetrs::block::Type::I);
    /// ```
    pub fn new(id: Type) -> Block {
        Block { x: 4,
                y: 0,
                id: id,
                r: Rotation::R0,
                rs: rotation::SRS{}
        }
    }

    /// Return `true` if the block collides with the field after applying the
    /// specified offset.
    fn collision_at(&self, field: &Field, (xo, yo): (i32, i32)) -> bool {
        self.rs.data(self.id, self.r).iter()
                  .map(|&(dx, dy)| {
                      (self.x + dx as i32 + xo, self.y + dy as i32 + yo)
                  })
                  .any(|(x, y)| {
                      let maxf = self.rs.max(self.id, self.r);
                      let minf = self.rs.min(self.id, self.r);

                      x - (minf.0 as i32) < 0 || x + maxf.0 as i32 > field.width as i32 ||
                      y - (minf.1 as i32) < 0 || y + maxf.1 as i32 > field.height as i32 ||
                      field.at((x as usize, y as usize)) != Type::None.to_usize()
                  })
    }

    /// Return `true` if the block currently collides with any pieces on the
    /// field.
    pub fn collision(&self, field: &Field) -> bool {
        self.collision_at(&field, (0, 0))
    }


    /// Shift the block by the specified coordinates.
    ///
    /// This does not check intermediate steps for collisions, so is not
    /// used for general multi-shifting.
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

    /// Shift the block one step in the specified direction.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::{Block, BlockBuilder, Type};
    /// use tetrs::Direction;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(Type::Z)
    ///                       .on_field(&field);
    /// block.shift(&field, Direction::Left);
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
    /// A HardDrop can be performed for example by calling
    /// `Block.shift_extend(&field, Direction::Down)`.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::{Block, BlockBuilder, Type};
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(Type::Z).on_field(&field);
    /// block.shift_extend(&field, tetrs::Direction::Left);
    /// ```
    pub fn shift_extend(&mut self, field: &Field, direction: Direction) {
        while self.shift(&field, direction) {}
    }

    /// Rotate the block by a specified amount and then apply an offset.
    ///
    /// This is useful for calculating wallkicks, where a collision check is
    /// done only after an offset is applied.
    ///
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::{Block, BlockBuilder, Type};
    /// use tetrs::Rotation;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(Type::Z).on_field(&field);
    ///
    /// // Rotate then move down 2 and right 1 and test for collision
    /// block.rotate_with_offset(&field, Rotation::R90, (2, 1));
    /// ```
    pub fn rotate_with_offset(&mut self, field: &Field, rotation: Rotation, (x, y): (i32, i32)) -> bool {
        let original_rotation = self.r;
        let new_rotation = match rotation {
            Rotation::R0   => self.r,
            Rotation::R90  => self.r.clockwise(),
            Rotation::R180 => self.r.clockwise().clockwise(),
            Rotation::R270 => self.r.anticlockwise()
        };

        self.r = new_rotation;

        if self.shift_raw(&field, (x, y)) {
            true
        }
        else {
            self.r = original_rotation;
            false
        }
    }

    /// Rotate the block by the specified amount.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::{Block, BlockBuilder, Type};
    /// use tetrs::Rotation;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(Type::Z).on_field(&field);
    /// block.rotate(&field, Rotation::R90);
    /// ```
    pub fn rotate(&mut self, field: &Field, rotation: Rotation) -> bool {
        self.rotate_with_offset(&field, rotation, (0, 0))
    }

    /// Check if the block occupies a particular `(x, y)` absolute location.
    pub fn at(&self, (a, b): (usize, usize)) -> bool {
        self.rs.data(self.id, self.r).iter()
            .map(|&(x, y)| (self.x as usize + x, self.y as usize + y))
            .any(|(x, y)| a == x && b == y)
    }


    /// Return a `Block` which is a ghost of the current.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    /// use tetrs::block::{Block, BlockBuilder};
    ///
    /// let field = Field::new();
    /// let block = Block::new(tetrs::block::Type::Z).on_field(&field);
    /// let ghost = block.ghost(&field);
    /// ```
    pub fn ghost(&self, field: &Field) -> Block {
        let mut ghost = self.clone();
        ghost.shift_extend(&field, Direction::Down);
        ghost
    }

    /// Return the used block data for the specified type.
    ///
    /// It may sometimes be useful to query block data without making a block
    /// instance itself. For example, when drawing preview pieces from only
    /// a `Type` value.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::block::{Block, Type};
    /// use tetrs::Rotation;
    ///
    /// let data = Block::data(Type::Z, Rotation::R270);
    /// ```
    pub fn data(id: Type, r: Rotation) -> &'static [(usize, usize)] {
        // Default to SRS for the moment, this should take another arg
        let srs = rotation::SRS{};
        srs.data(id, r)
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
        let mut block = Block::new(Type::Z)
                              .on_field(&field);

        let x = block.x;
        block.shift(&field, Direction::Left);
        assert_eq!(x - 1, block.x);

        block.shift_extend(&field, Direction::Left);
        assert_eq!(0, block.x);
    }

    #[test]
    fn test_rotation() {
        let field = Field::new();
        let mut block = Block::new(Type::S)
                              .on_field(&field);

        block.shift(&field, Direction::Down);
        block.shift(&field, Direction::Down);

        block.rotate(&field, Rotation::R90);
        assert_eq!(block.r, Rotation::R90);

        block.rotate(&field, Rotation::R90);
        assert_eq!(block.r, Rotation::R180);

        block.rotate(&field, Rotation::R270);
        assert_eq!(block.r, Rotation::R90);
    }
}
