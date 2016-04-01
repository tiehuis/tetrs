#![feature(collections, enumset)]
#![warn(missing_docs)]

#![crate_name = "tetrs"]

//! The tetrs library provides a number of low-level tasks related to movement
//! of blocks. The code aims to be correct and provide easy extension for new
//! input.
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
//pub mod engine;

pub mod schema;


/// Represents all rotation statuses a block can be. This is used both as
/// a rotation state, and to indicate how much relative movement shoud be
/// applied for various functions.
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
    /// Returns all rotation variants
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

/// Specifies a particular direction of movement.
#[repr(usize)]
#[derive(Hash, Clone, Debug, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Direction {
    None, Left, Right, Up, Down
}

impl Direction {
    /// It is required to be able to iterate over enums in some places. These
    /// enums are simple CLike so it isn't too weird to think we would need to
    /// do this, but it still provides some problems.
    pub fn variants() -> Vec<Direction> {
        vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down]
    }
}
