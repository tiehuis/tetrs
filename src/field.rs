//! A generic playfield.
//!
//! A `Field` stores the state of previously placed blocks, and is used to
//! determine collisions for any `Block`. A `Field` is not actually aware
//! of what blocks belong to it, tis generality allows for more advanced
//! gameplay to be built from this structure.

use block;
use block::Block;
use rotation::RotationSystem;

use std::iter;
use itertools::Itertools;
use collections::enum_set::CLike;

/// A `Field` is simply a 2-D `Vec` with some corresponding options.
///
/// Internally we use a `Vec` of `Vec`'s for easier resizing, but all rows
/// must be of equal length, so it can be thought of as rectangular for all
/// purposes.
#[derive(Hash, Clone, Debug)]
pub struct Field {
    /// The width of the field.
    pub width: usize,

    /// The height of the field.
    pub height: usize,

    /// The height of the hidden region of the field.
    ///
    /// This is a subregion of height, and does not actually add any more
    /// height to the field.
    pub hidden: usize,

    /// The initial spawn of a `Block` on this field.
    pub spawn: (usize, usize),

    /// The current field state.
    pub data: Vec<Vec<usize>>,
}

/// Handles building of more complicated fields than can be constructed with
/// `new` itself.
///
/// More options may be added in the future whilst keeping
/// backwards-compatibility.
///
/// ## Examples
/// ```
/// use tetrs::field::{Field, FieldBuilder};
///
/// // Constructs a field with a width of 12 and height of 30
/// let field = Field::new().width(12).height(30);
/// ```
pub trait FieldBuilder {
    /// Alter the width of the field and return the modified field.
    fn width(self, width: usize) -> Field;

    /// Alter the height of the field and return the modified field.
    fn height(self, height: usize) -> Field;

    /// Alter the hidden height of the field and return the modified field.
    fn hidden(self, hidden: usize) -> Field;

    /// Alter the block spawn point of the field and return the modified.
    fn spawn(self, spawn: (usize, usize)) -> Field;
}

/// Not sure if this would be better as an actual type to disallow calling
/// these functions on arbitrary fields, but we can trust the caller for now.
impl FieldBuilder for Field {
    fn width(mut self, width: usize) -> Field {
        self.width = width;
        self.data.iter_mut().foreach(|x| x.resize(width, block::Type::None.to_usize()));
        self
    }

    fn height(mut self, height: usize) -> Field {
        assert!(height > self.hidden);
        let width = self.width;
        self.height = height;
        self.data.resize(height, vec![block::Type::None.to_usize(); width]);
        self
    }

    fn hidden(mut self, hidden: usize) -> Field {
        self.hidden = hidden;
        self
    }

    fn spawn(mut self, spawn: (usize, usize)) -> Field {
        self.spawn = spawn;
        self
    }
}

impl Field {
    /// Construct a new field object.
    ///
    /// This is often combined with the `FieldBuilder` trait impl's to provide
    /// custom parameters for a `Field`.
    ///
    /// A `Field` can be constructed with no arguments, in which case the
    /// following defaults are used:
    ///
    /// ## Width
    /// Default width is 10 columns.
    ///
    /// ## Height
    /// Default height is 25 rows.
    ///
    /// ## Hidden
    /// The hidden segment takes up 3 rows.
    ///
    /// ## Spawn
    /// The default spawn is at coordinates `(4, 0)`.
    ///
    /// ---
    ///
    /// If values are to be overridden, look at the `FieldBuilder` trait impl.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    ///
    /// let field = Field::new();
    /// ```
    pub fn new() -> Field {
        Field {
            width: 10,
            height: 25,
            hidden: 3,
            spawn: (4, 0),
            data: vec![vec![block::Type::None.to_usize(); 25]; 10]
        }
    }

    /// Clear lines from the field and return the number cleared.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    ///
    /// let mut field = Field::new();
    /// let lines_cleared = field.clear_lines();
    /// ```
    pub fn clear_lines(&mut self) -> usize {
        // Clear all filled lines
        self.data.retain(|ref x| x.iter().all(|&x| x != block::Type::None.to_usize()));

        // Calculate how many lines were cleared
        let lines = self.height - self.data.len();

        // Count the lines cleared and add new empty rows to the end
        self.data.extend(iter::repeat(vec![block::Type::None.to_usize(); self.width]).take(lines));

        lines
    }

    /// Freeze a block into place on the field. This takes ownership of the
    /// block to ensure it cannot be used again.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::{Field};
    /// use tetrs::block::{Block, Type};
    /// use tetrs::Direction;
    ///
    /// let mut field = Field::new();
    /// let mut block = Block::new(Type::I);
    ///
    /// field.freeze(block);
    ///
    /// // block.shift(Direction::Right); // Compile Error
    /// ```
    pub fn freeze(&mut self, block: Block) {
        let data = block.rs.data(block.id, block.r);
        data.iter()
            .enumerate()
            .map(|(i, &(x, y))| (x + data[i].0, y + data[i].1))
            .foreach(|(x, y)| {
                self.data[x][y] = block.id.to_usize();
            });
    }

    /// Return the value at the specified field location.
    ///
    /// This currently is a `usize` value which corresponds to some `Type`
    /// of `Block`.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    ///
    /// let field = Field::new();
    /// let value = field.at((5, 10));
    /// ```
    pub fn at(&self, (x, y): (usize, usize)) -> usize {
        assert!(x < self.width && y < self.height);
        self.data[x][y]
    }

    /// Return true if the value at the specified location is non-empty.
    ///
    /// This is a convenience function which queries `at` and checks if the
    /// result is of empty `Type`.
    pub fn set(&self, (x, y): (usize, usize)) -> bool {
        assert!(x < self.width && y < self.height);
        self.data[x][y] != block::Type::None.to_usize()
    }
}
