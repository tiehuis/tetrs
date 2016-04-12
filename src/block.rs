//! A single tetrimino.
//!
//! A `Block` consists of a `Type`, `Location` and `Rotation`. It does not
//! have to be considered part of a `Field`, however most movement functions
//! take a `Field` with which movement is verified against.
//!
//! ```ignore
//! use tetrs::{Rotation, Direction};
//! use tetrs::field::Field;
//! use tetrs::block::{Block, BlockOptions};
//!
//! let field = Field::new();
//!
//! // Can spawn a block on a field directly...
//! let block = Block::new().set_field(&field);
//!
//! // ...or independently
//! let block = Block::new().set_position((5, 10));
//! ```

use field::Field;
use rotation_system::{self, RotationSystem};

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
    /// use tetrs::import::*;
    ///
    /// let types = block::Type::variants();
    /// ```
    pub fn variants() -> &'static [Type] {
        static VARIANTS: &'static [Type; 7] = &[
            Type::I, Type::T, Type::L, Type::J, Type::S, Type::Z, Type::O
        ];

        VARIANTS
    }
}

/// Represents all rotation statuses a block can be. This is used both as
/// a rotation state, and to indicate how much relative movement shoud be
/// applied for various functions.
/// A rotation state.
///
/// Rotations states are available in 90 degree increments and internally all
/// follow a clockwise direction.
///
/// ## Examples
/// ```ignore
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
    /// Returns all available `Rotation` variants.
    ///
    /// ```
    /// use tetrs::import::*;
    ///
    /// let rotations = Rotation::variants();
    /// ```
    pub fn variants() -> Vec<Rotation> {
        vec![Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270]
    }

    /// Returns the next clockwise rotation.
    ///
    /// ```
    /// use tetrs::import::*;
    ///
    /// let rotation = Rotation::R0;
    /// assert_eq!(rotation.clockwise(), Rotation::R90);
    /// ```
    pub fn clockwise(&self) -> Rotation {
        Rotation::from_usize((self.to_usize() + 4 + 1) % 4)
    }

    /// Returns the next anticlockwise rotation.
    ///
    /// ```
    /// use tetrs::import::*;
    ///
    /// let rotation = Rotation::R270;
    /// assert_eq!(rotation.anticlockwise(), Rotation::R180);
    /// ```
    pub fn anticlockwise(&self) -> Rotation {
        Rotation::from_usize((self.to_usize() + 4 - 1) % 4)
    }
}

/// A movement along one of the four directional axes.
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Direction {
    None, Left, Right, Up, Down
}

impl Direction {
    /// Return all `non-None` `Direction` variants.
    ///
    /// ```
    /// let directions = tetrs::block::Direction::variants();
    /// ```
    pub fn variants() -> Vec<Direction> {
        vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]
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
#[derive(Clone)]
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
    // All Rotation Systems are not instatiated directly, but use a static
    // instance. This makes storing the trait object much easier and ensures
    // that we use the builder properly.
    //
    // A `Block` requires a `RotationSystem` since moving and dealing with
    // blocks is an integral part. This could be argued the same as well for
    // a `Field`, which may happen.
    pub rs: &'static RotationSystem
}

/// Values used for initializing a block.
///
/// This is similar to a builder structure, but instead of chained method calls
/// we rely on the `Default` trait.
///
/// This can be used to initialize a block with more fine-grained details than
/// standard, whilst retaining useful defaults of values which may not want to
/// be changed.
///
/// ```ignore
/// // Create an option set with default options besides the initial rotation
/// let bopt = BlockOptions { rotation: Rotation::R90, ..Default::default() }
/// ```
#[allow(missing_docs)]
pub struct BlockOptions {
    pub x: Option<i32>,

    pub y: Option<i32>,

    pub rotation: Rotation,

    pub rotation_system: &'static str
}

impl Default for BlockOptions {
    fn default() -> BlockOptions {
        BlockOptions {
            x: None,
            y: None,
            rotation: Rotation::R0,
            rotation_system: "srs"
        }
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
    /// If values are required to be overridden, look at the `with_options`
    /// function.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let block = Block::new(block::Type::I, &field);
    /// ```
    pub fn new(id: Type, field: &Field) -> Block {
        Block::with_options(id, &field, BlockOptions { ..Default::default() })
    }

    /// Construct a new block with the specified options.
    pub fn with_options(id: Type, field: &Field, options: BlockOptions) -> Block {
        Block {
            id: id,
            x: if options.x.is_none() { field.spawn.0 } else { options.x.unwrap() },
            y: if options.y.is_none() { field.spawn.1 } else { options.y.unwrap() },
            r: options.rotation,
            rs: rotation_system::new(options.rotation_system)
        }
    }

    /// Return `true` if the block collides with the field after applying the
    /// specified offset.
    fn collides_at_offset(&self, field: &Field, (xo, yo): (i32, i32)) -> bool {
        self.rs.data(self.id, self.r).iter()
                  .map(|&(dx, dy)| {
                      (self.x + i32!(dx) + xo, self.y + i32!(dy) + yo)
                  })
                  .any(|(x, y)| {
                      x < 0 || usize!(x) >= field.width ||
                      y < 0 || usize!(y) >= field.height ||
                      field.get((usize!(x), usize!(y))) != Type::None.to_usize()
                  })
    }

    /// Return `true` if the block currently collides with any pieces on the
    /// field.
    pub fn collides(&self, field: &Field) -> bool {
        self.collides_at_offset(&field, (0, 0))
    }


    /// Shift the block by the specified coordinates.
    ///
    /// This does not check intermediate steps for collisions, so is not
    /// used for general multi-shifting.
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
    /// let mut block = Block::new(block::Type::Z, &field);
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
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Type::Z, &field);
    /// block.shift_extend(&field, Direction::Left);
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
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Type::Z, &field);
    ///
    /// // Rotate then move down 2 and right 1 and test for collision
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
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Type::Z, &field);
    /// block.rotate(&field, Rotation::R90);
    /// ```
    pub fn rotate(&mut self, field: &Field, rotation: Rotation) -> bool {
        self.rotate_at_offset(&field, rotation, (0, 0))
    }

    /// Check if the block occupies a particular `(x, y)` absolute location.
    pub fn occupies(&self, (a, b): (usize, usize)) -> bool {
        self.rs.data(self.id, self.r).iter()
            .map(|&(x, y)| (self.x + i32!(x), self.y + i32!(y)))
            .any(|(x, y)| a == usize!(x) && b == usize!(y))
    }


    /// Return a `Block` which is a ghost of the current.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let block = Block::new(block::Type::Z, &field);
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
        let mut block = Block::new(Type::Z, &field);

        let x = block.x;
        block.shift(&field, Direction::Left);
        assert_eq!(x - 1, block.x);

        block.shift_extend(&field, Direction::Left);
        assert_eq!(0, block.x);
    }

    #[test]
    fn test_rotation() {
        let field = Field::new();
        let mut block = Block::new(Type::S, &field);

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
