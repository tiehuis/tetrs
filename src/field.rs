//! A generic playfield.
//!
//! A `Field` stores the state of previously placed blocks, and is used to
//! determine collisions for any `Block`. A `Field` is not actually aware
//! of what blocks belong to it, tis generality allows for more advanced
//! gameplay to be built from this structure.

use block::{self, Block};
use rotation::RotationSystem;

use itertools::Itertools;
use collections::enum_set::CLike;

/// A `Field` is simply a 2-D `Vec` with some corresponding options.
///
/// Internally we use a `Vec` of `Vec`'s for easier resizing, but all rows
/// must be of equal length, so it can be thought of as rectangular for all
/// purposes.
///
/// ## Note
///
/// If one wants to only iterate over the viewable portion of the array, this
/// can be done with the following bounds:
///
/// ```ignore
/// for y in self.hidden..self.height {
///     ...
/// }
/// ```
///
/// instead of the more presumed
///
/// ```ignore
/// for y in 0..self.height {
/// }
/// ```
///
/// This is because the `height` field includes the specified `hidden` portion.
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
    pub spawn: (i32, i32),

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
/// let field = Field::new().set_width(12).set_height(30);
/// ```
pub trait FieldBuilder {
    /// Alter the width of the field and return the modified field.
    fn set_width(self, width: usize) -> Field;

    /// Alter the height of the field and return the modified field.
    fn set_height(self, height: usize) -> Field;

    /// Alter the hidden height of the field and return the modified field.
    fn set_hidden(self, hidden: usize) -> Field;

    /// Alter the block spawn point of the field and return the modified.
    fn set_spawn(self, spawn: (i32, i32)) -> Field;
}

impl FieldBuilder for Field {
    fn set_width(mut self, width: usize) -> Field {
        self.width = width;
        self.data.iter_mut().foreach(|x| x.resize(width, block::Type::None.to_usize()));
        self
    }

    fn set_height(mut self, height: usize) -> Field {
        assert!(height > self.hidden);
        let width = self.width;
        self.height = height;
        self.data.resize(height, vec![block::Type::None.to_usize(); width]);
        self
    }

    fn set_hidden(mut self, hidden: usize) -> Field {
        self.hidden = hidden;
        self
    }

    fn set_spawn(mut self, spawn: (i32, i32)) -> Field {
        self.spawn = spawn;
        self
    }
}

impl Default for Field {
    fn default() -> Field {
        Field {
            width: 10,
            height: 25,
            hidden: 3,
            spawn: (4, 0),
            data: vec![vec![block::Type::None.to_usize(); 10]; 25]
        }
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
        Field { ..Default::default() }
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
        // Keep only lines with an empty cell (non-filled)
        self.data.retain(|ref x| x.iter().any(|&x| x == block::Type::None.to_usize()));

        // Calculate how many lines were cleared
        let lines = self.height - self.data.len();

        // Sure this isn't optimal, but for a small array and with only 4
        // pushses max (unless cascading) who would notice?
        for _ in 0..lines {
            self.data.insert(0, vec![block::Type::None.to_usize(); self.width]);
        }

        lines
    }

    /// Freeze a block into place on the field. This takes ownership of the
    /// block to ensure it cannot be used again.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::{Field};
    /// use tetrs::block::{Block, Direction, Type};
    ///
    /// let mut field = Field::new();
    /// let mut block = Block::new(Type::I);
    ///
    /// field.freeze(block);
    ///
    /// // block.shift(Direction::Right); // Compile Error
    /// ```
    pub fn freeze(&mut self, block: Block) {
        for &(x, y) in block.rs.data(block.id, block.r) {
            self.data[usize!(block.y + i32!(y))][usize!(block.x + i32!(x))] = block.id.to_usize();
        }
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
    /// let value = field.get((5, 10));
    /// ```
    pub fn get(&self, (x, y): (usize, usize)) -> usize {
        assert!(x < self.width && y < self.height);
        self.data[y][x]
    }

    /// Return true if the value at the specified location is non-empty.
    ///
    /// This is a convenience function which queries `at` and checks if the
    /// result is of empty `Type`.
    pub fn occupies(&self, (x, y): (usize, usize)) -> bool {
        assert!(x < self.width && y < self.height);
        self.data[y][x] != block::Type::None.to_usize()
    }
}
