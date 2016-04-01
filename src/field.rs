//! A generic field for use with a game.
//!
//! The field has little concept about any other components and has no
//! dependencies on them.

use block;
use block::Block;

use std::iter;
use itertools::Itertools;
use collections::enum_set::CLike;

/// Implements a field state with specified dimensions and game data.
#[derive(Hash, Clone, Debug)]
pub struct Field {
    /// The width of the field.
    pub width: usize,

    /// The height of the field.
    pub height: usize,

    /// The height of the hidden region of the field.
    pub hidden: usize,

    /// The initial spawn of all blocks.
    pub spawn: (usize, usize),

    /// The current field state.
    pub data: Vec<Vec<usize>>,
}

impl Field {
    /// Construct a new field object.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    ///
    /// let field = Field::new();
    /// ```
    pub fn new() -> Field {
        Field {
            width: 10, height: 25, hidden: 3, spawn: (3, 0),
            data: vec![vec![block::Type::None.to_usize(); 25]; 10]
        }
    }

    /// Construct a new field object with the specified options.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::field::Field;
    ///
    /// let field = Field::with_options(12, 25, (5, 0));
    /// ```
    pub fn with_options(width: usize, height: usize, spawn: (usize, usize))
            -> Field {
        Field  {
            width: width, height: height, hidden: 3, spawn: (spawn.0, spawn.1),
            data: vec![vec![block::Type::None.to_usize(); height]; width]
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
    /// use tetrs::{field, block};
    ///
    /// let mut field = field::Field::new();
    /// let mut block = block::Block::new(block::Type::I, tetrs::Rotation::R0);
    ///
    /// field.freeze(block);
    ///
    /// // block.shift(tetrs::Direction::Right); // Invalid
    /// ```
    pub fn freeze(&mut self, block: Block) {
        block.data.iter()
            .enumerate()
            .map(|(i, &(x, y))| (x + block.data[i].0, y + block.data[i].1))
            .foreach(|(x, y)| {
                self.data[x][y] = block.id.to_usize();
            });
    }

    /// Return the value at the specified field location
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

    /// Return if the value at the specified location is empty or not.
    pub fn set(&self, (x, y): (usize, usize)) -> bool {
        assert!(x < self.width && y < self.height);
        self.data[x][y] != block::Type::None.to_usize()
    }
}
