#![feature(collections, enumset)]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![warn(missing_docs)]

#![crate_name = "tetrs"]
#![doc(html_root_url = "https://tiehuis/github.io/tetrs/tetrs/")]

//! The tetrs library provides a number of low-level tasks for required for
//! tetris gameplay. This library aims to be correct and provide easy extension
//! for new frontends/rotation-systems etc.
//!
//! Intra-module dependencies are reduced as much as possible. If a component
//! does not require knowledge of another component, then it is likely not
//! present. Some helper functions which do not adhere to this philosophy can
//! be located in the `utility` module.
//!
//! Finally, a fairly general high-level abstraction over these is provided
//! with the `engine` module.
//!
//! ## Examples
//!
//! ```
//! use tetrs::import::*;
//!
//! let field = Field::new();
//! let mut block = Block::new(block::Id::I, &field);
//!
//! block.rotate(&field, Rotation::R90);
//! block.shift_extend(&field, Direction::Down);
//! ```

extern crate serde_json;
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

#[macro_use]
pub mod schema;

pub mod field;
pub mod block;
pub mod controller;
pub mod wallkick;
pub mod randomizer;
pub mod rotation_system;
pub mod engine;
pub mod utility;
pub mod statistics;
pub mod import;
pub mod history;
