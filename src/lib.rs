#![feature(collections, enumset, plugin)]

#![plugin(clippy)]
#![warn(missing_docs)]

#![crate_name = "tetrs"]
#![doc(html_root_url = "https://tiehuis/github.io/tetrs/tetrs/")]

//! The tetrs library provides a number of low-level tasks related to movement
//! of blocks. The code aims to be correct and provide easy extension for new
//! input.
//!
//! ## Examples
//!
//! ```ignore
//! use tetrs::{block, field, Rotation, Direction};
//!
//! let field = field::new().width(12);
//! let block = block::new().on_field(&field);
//!
//! block.rotate(&field, Rotation::R90);
//! block.shift_extend(&field, Direction::Down);
//! ```
//!
//! Intra-module dependencies are as reduced in scope as possible. For example,
//! a block itself has no knowledge of a wallkick, but provides functionality
//! in terms of rotations with offset to allow for easy use with wallkicks.
//!
//! A higher-level abstraction in terms of a game-engine is provided. This
//! provides DAS-like behaviour and such.

extern crate collections;
#[macro_use] extern crate itertools;
extern crate rand;

pub mod field;
pub mod block;
pub mod controller;
pub mod wallkick;
pub mod randomizer;
pub mod rotation;
pub mod engine;

pub mod schema;

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

use collections::enum_set::CLike;

impl CLike for Rotation {
    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn from_usize(v: usize) -> Rotation {
        assert!(v < 4);
        unsafe { std::mem::transmute(v) }
    }
}

impl Rotation {
    /// Returns all available `Rotation` variants.
    ///
    /// ```
    /// let rotations = tetrs::Rotation::variants();
    /// ```
    pub fn variants() -> Vec<Rotation> {
        vec![Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270]
    }

    /// Returns the next clockwise rotation.
    ///
    /// ```
    /// use tetrs::Rotation;
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
    /// use tetrs::Rotation;
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
    /// let directions = tetrs::Direction::variants();
    /// ```
    pub fn variants() -> Vec<Direction> {
        vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]
    }
}
