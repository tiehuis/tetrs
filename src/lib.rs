#![feature(collections, enumset)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

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
extern crate time;
#[macro_use] extern crate log;

/// Perform a safe conversion to i32, panicing if the current type does not
/// lie within the required bounds.
macro_rules! usize {
    ($x:expr) => {
        if $x < 0 {
            panic!("cannot construct usize from value: {}", $x);
        }
        else {
            $x as usize
        }
    }
}

macro_rules! i32 {
    ($x:expr) => {
        $x as i32
    }
}

pub mod field;
pub mod block;
pub mod controller;
pub mod wallkick;
pub mod randomizer;
pub mod rotation;
pub mod engine;
pub mod utility;
pub mod options;
pub mod statistics;
pub mod schema;
