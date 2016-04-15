//! Defines a single tetrimino and actions for modifying it.
//!
//! A `Block` consists of a type, location and rotation. Each block does not
//! store an internal reference to a `Field`, but it is required in most
//! useful functions.
//!
//! ```
//! use tetrs::import::*;
//!
//! let field = Field::new();
//! // A block must be spawned on a field
//! let block1 = Block::new(block::Id::I, &field);
//!
//! // This position can be overridden if required (still requires a field)
//! let block2 = Block::with_options(block::Id::I, &field, BlockOptions {
//!                 x: Some(3), y: Some(4),
//!                 ..Default::default()
//!             });
//! ```

use field::Field;
use rotation_system::{self, RotationSystem};

use std::mem;
use collections::enum_set::CLike;

/// The identifier for a particular `Block`.
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Id {
    I, T, L, J, S, Z, O, None
}

impl CLike for Id {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(v: usize) -> Id {
        assert!(v < 7);
        unsafe { mem::transmute(v) }
    }
}

impl Id {
    /// Returns all `Id` variants known.
    ///
    /// This does not return the `None` variant.
    pub fn variants() -> &'static [Id] {
        static VARIANTS: &'static [Id; 7] = &[
            Id::I, Id::T, Id::L, Id::J, Id::S, Id::Z, Id::O
        ];

        VARIANTS
    }
}

/// Represents all rotation statuses a block can be. This is used both as
/// a rotation state, and to indicate how much relative movement shoud be
/// applied for various functions.
///
/// Rotations states are available in 90 degree increments and internally all
/// follow a clockwise direction.
///
/// ## Examples
/// ```text
///
/// Rotation::R90  Rotation::R270
///
///   ---------      ---------
///   |       |      |       |
///   |   .-->|      |<--.   |
///   |       |      |       |
///   ---------      ---------
/// ```
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Rotation {
    R0, R90, R180, R270
}

impl CLike for Rotation {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(v: usize) -> Rotation {
        assert!(v < 4);
        unsafe { mem::transmute(v) }
    }
}

impl Rotation {
    /// Returns all known `Rotation` variants.
    pub fn variants() -> Vec<Rotation> {
        vec![Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270]
    }

    /// Returns the next clockwise rotation.
    pub fn clockwise(&self) -> Rotation {
        Rotation::from_usize((self.to_usize() + 4 + 1) % 4)
    }

    /// Returns the next anticlockwise rotation.
    pub fn anticlockwise(&self) -> Rotation {
        Rotation::from_usize((self.to_usize() + 4 - 1) % 4)
    }
}

/// A movement along one of the four directional axes.
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Direction {
    Left, Right, Up, Down
}

impl Direction {
    /// Return all known `Direction` variants.
    pub fn variants() -> Vec<Direction> {
        vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]
    }
}

/// A struct representing a single tetrimino.
///
/// Blocks are defined by an `(x, y)` coordinate pair, a `Rotation`, `Id`,
/// and an accompanying set of offset data which specifies exactly which
/// pieces in the field this block occupies.
///
/// Internally, `(x, y)` coordinates are taken with their origin from the top
/// right of the grid, to preserve some compatibility with GUI systems. All
/// internal block calculations follow this as well:
///
/// ```text
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
/// ```text
///   (x, y) = (4, 4)
///     data = [(1, 0), (0, 1), (1, 1), (2, 1)]
/// ```
///
/// When calculating a block's position, the data offsets are added to the
/// base coordinates to produce the block. The previous block is a `Id::T`,
/// with `Rotation::R0`, as the final data represented is:
///
/// ```text
///  block = [(5, 4), (4, 5), (5, 5), (6, 5)]
/// ```
///
/// It is important to note that offsets are all non-negative, and as such the
/// coordinate effectively partitions the space where the block can reside.
/// Further, a block could have many different internal representations which
/// appear equal, by adjusting the `(x, y)` coordinates and `data` in
/// conjunction.
#[derive(Clone)]
pub struct Block {
    /// X-coordinate of the piece
    pub x:  i32,

    /// Y-coordinate of the piece
    pub y:  i32,

    /// Id of the block
    pub id: Id,

    /// Rotation state of the block
    pub r: Rotation,

    /// Rotation system used to calculate block offsets.
    pub rs: &'static RotationSystem
}

/// Optional values which can be set when initializing a `Block`.
///
/// The default values are:
///
/// ```text
/// BlockOptions {
///     x: None,
///     y: None,
///     rotation: Rotation::R0,
///     rotation_system: rotation_system::SRS
/// }
/// ```
///
/// If an `x` or `y` value is `Some(..)` then it will override the fields spawn
/// position, which is usually used on `Block` construction.
///
/// ## Examples
///
/// ```
/// use tetrs::import::*;
///
/// // Has x: None, y: None, rotation: Rotation::R0, rotation_system: "dtet"
/// let options = BlockOptions {
///     rotation_system: rotation_system::new("dtet"),
///     ..Default::default()
/// };
/// ```
#[allow(missing_docs)]
pub struct BlockOptions {
    pub x: Option<i32>,

    pub y: Option<i32>,

    pub rotation: Rotation,

    pub rotation_system: &'static RotationSystem
}

impl Default for BlockOptions {
    fn default() -> BlockOptions {
        BlockOptions {
            x: None,
            y: None,
            rotation: Rotation::R0,
            rotation_system: rotation_system::new("srs")
        }
    }
}

impl Block {
    /// Construct a `Block` object with default values.
    pub fn new(id: Id, field: &Field) -> Block {
        Block::with_options(id, &field, BlockOptions { ..Default::default() })
    }

    /// Construct a `Block` object with specific values.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let block = Block::with_options(block::Id::I, &field, BlockOptions {
    ///                 rotation: Rotation::R180,
    ///                 ..Default::default()
    ///             });
    /// ```
    pub fn with_options(id: Id, field: &Field, options: BlockOptions) -> Block {
        Block {
            id: id,
            x: if options.x.is_none() { field.spawn.0 } else { options.x.unwrap() },
            y: if options.y.is_none() { field.spawn.1 } else { options.y.unwrap() },
            r: options.rotation,
            rs: options.rotation_system
        }
    }

    /// Return whether the block collides with `field` at the specified offset.
    fn collides_at_offset(&self, field: &Field, (xo, yo): (i32, i32)) -> bool {
        self.rs.data(self.id, self.r).iter()
                  .map(|&(dx, dy)| {
                      (self.x + i32!(dx) + xo, self.y + i32!(dy) + yo)
                  })
                  .any(|(x, y)| {
                      x < 0 || usize!(x) >= field.width ||
                      y < 0 || usize!(y) >= field.height ||
                      field.get((usize!(x), usize!(y))) != Id::None
                  })
    }

    /// Return whether the current `Block` collides with `field` at its current
    /// position.
    pub fn collides(&self, field: &Field) -> bool {
        self.collides_at_offset(&field, (0, 0))
    }

    /// Shift the block by the specified coordinates.
    ///
    /// ## Note
    /// This does not check intermediate steps for collisions.
    fn shift_raw(&mut self, field: &Field, (x, y): (i32, i32)) -> bool {
        if self.collides_at_offset(&field, (x, y)) {
            false
        }
        else {
            self.x += x;
            self.y += y;
            true
        }
    }

    /// Shift the block one step in the specified direction.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Id::Z, &field);
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
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Id::Z, &field);
    ///
    /// // Performs a 'HardDrop'
    /// block.shift_extend(&field, Direction::Down);
    /// ```
    pub fn shift_extend(&mut self, field: &Field, direction: Direction) {
        while self.shift(&field, direction) {}
    }

    /// Rotate the block by a specified amount and then apply an offset.
    ///
    /// This is useful for calculating wallkicks. See the `rotate_with_wallkick`
    /// function in the `utility` module for an easier function.
    ///
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Id::Z, &field);
    ///
    /// // Rotate then move down 2 and right 1 and test for collision.
    /// // If we collide, then do not move the piece.
    /// block.rotate_at_offset(&field, Rotation::R90, (2, 1));
    /// ```
    pub fn rotate_at_offset(&mut self, field: &Field, rotation: Rotation, (x, y): (i32, i32)) -> bool {
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
    pub fn rotate(&mut self, field: &Field, rotation: Rotation) -> bool {
        self.rotate_at_offset(&field, rotation, (0, 0))
    }

    /// Check if the block occupies a particular `(x, y)` absolute location.
    pub fn occupies(&self, (a, b): (usize, usize)) -> bool {
        self.rs.data(self.id, self.r).iter()
            .map(|&(x, y)| (self.x + i32!(x), self.y + i32!(y)))
            .any(|(x, y)| a == usize!(x) && b == usize!(y))
    }


    /// Returns a new `Block` which is the current blocks ghost.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let block = Block::new(block::Id::Z, &field);
    /// let ghost = block.ghost(&field);
    /// ```
    pub fn ghost(&self, field: &Field) -> Block {
        let mut ghost = self.clone();
        ghost.shift_extend(&field, Direction::Down);
        ghost
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use field::Field;

    #[test]
    fn test_shift() {
        let field = Field::new();
        let mut block = Block::new(Id::Z, &field);

        let x = block.x;
        block.shift(&field, Direction::Left);
        assert_eq!(x - 1, block.x);

        block.shift_extend(&field, Direction::Left);
        assert_eq!(0, block.x);
    }

    #[test]
    fn test_rotation() {
        let field = Field::new();
        let mut block = Block::new(Id::S, &field);

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
