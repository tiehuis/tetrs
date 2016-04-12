//! Implements a wallkick test
//!
//! A wallkick test returns the number of rotation offsets that should be
//! attempted on rotation failure.
//!
//! It should be implemented as an iterator (currently a 'static vector)
//! which returns (i32, i32) tuples.

use block::{Block, Rotation};

/// Trait which specifies what wallkick tests must implement. Every wallkick
/// test must implement an iterator with offsets of type (i32, i32).
pub trait Wallkick {
    /// Returns a set of wallkick tests for the specified block and rotation
    fn test(&self, block: &Block, r: Rotation) -> &'static [(i32, i32)];
}

macro_rules! gen_wallkick {
    ($wkid:ident) => {
        /// Wallkick
        pub struct $wkid {}

        static __INSTANCE: $wkid = $wkid {};

        impl $wkid {
            /// Return a new wallkick instance
            pub fn new() -> &'static $wkid {
                &__INSTANCE
            }
        }
    }
}

pub use self::srs::SRS;
mod srs;

pub use self::empty::Empty;
mod empty;

pub use self::simple::Simple;
mod simple;

pub use self::dtet::DTET;
mod dtet;

/// Factory function for returning appropriate rotation systems based on their
/// names
pub fn new(name: &str) -> &'static Wallkick {
    match name {
        "srs" => SRS::new(),
        "empty" => Empty::new(),
        "simple" => Simple::new(),
        "dtet" => DTET::new(),
        _ => panic!("unknown wallkick")
    }
}
