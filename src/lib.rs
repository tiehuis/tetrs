#![feature(collections, enumset)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

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
//! use tetrs::import::*;
//!
//! let field = Field::new();
//! let block = Block::new(block::Type::I, &field);
//!
//! block.rotate(&field, Rotation::R90);
//! block.shift_extend(&field, Direction::Down);
//! ```
//!
//! Intra-module dependencies are as reduced in scope as possible. For example,
//! a block itself has no knowledge of a wallkick, but provides functionality
//! in terms of rotations with offset to allow for easy use with wallkicks.
//!
//! A high-level abstraction over these primitives is provided in terms of an
//! `Engine` class.

extern crate collections;
#[macro_use] extern crate itertools;
extern crate rand;
extern crate time;
#[macro_use] extern crate log;
extern crate serde;
extern crate serde_json;

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
pub mod rotation_system;
pub mod engine;
pub mod utility;
pub mod statistics;
pub mod schema;
pub mod import;
