//! A generic playfield.
//!
//! A `Field` stores the state of previously placed blocks, and is used to
//! determine collisions for any `Block`. A `Field` is not actually aware
//! of what blocks belong to it, tis generality allows for more advanced
//! gameplay to be built from this structure.

use block::{self, Block};
use rotation_system::RotationSystem;

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

/// Options for use when constructing a field.
#[derive(Serialize, Deserialize, Debug)]
#[allow(missing_docs)]
pub struct FieldOptions {
    pub width: usize,

    pub height: usize,

    pub hidden: usize,

    pub spawn: (i32, i32)
}

impl Default for FieldOptions {
    fn default() -> FieldOptions {
        FieldOptions {
            width: 10,
            height: 25,
            hidden: 3,
            spawn: (4, 0)
        }
    }
}

impl Field {
    /// Construct a `Field` object with default values.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// ```
    pub fn new() -> Field {
        Field::with_options(FieldOptions { ..Default::default() })
    }

    /// Construct a `Field` object with specific values.
    ///
    /// ## Examples
    ///
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::with_options(FieldOptions {
    ///                 width: 15, height: 22,
    ///                 ..Default::default()
    ///             });
    /// ```
    pub fn with_options(options: FieldOptions) -> Field {
        Field {
            width: options.width,
            height: options.height,
            hidden: options.hidden,
            spawn: options.spawn,
            data: vec![vec![block::Type::None.to_usize(); options.width]; options.height]
        }
    }

    /// Clear lines from the field and return the number cleared.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
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
    /// use tetrs::import::*;
    ///
    /// let mut field = Field::new();
    /// let mut block = Block::new(block::Type::I, &field);
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
    /// use tetrs::import::*;
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
