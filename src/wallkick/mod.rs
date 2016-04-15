//! A module for specifying wallkick tests.
//!
//! A wallkick test simply returns a number of rotational offsets that
//! should be attempted on rotation failure.
//!
//! User wallkicks can be implemented, and for the most part, only require
//! the `Wallkick` trait to be implemented.

use block::{Block, Rotation};
use field::Field;

/// The `Wallkick` trait must be implemented by all wallkicks.
///
/// Since wallkicks deal with static data, they often do not require an actual
/// instance and instead static references are preferred.
pub trait Wallkick {
    /// Wallkick tests for the specified id and rotation.
    ///
    /// Wallkick test values are expected to lie in a static array currently.
    /// This may be changed in the future if something is more applicable.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let field = Field::new();
    /// let mut block = Block::new(block::Id::Z, &field);
    /// let wallkick = wallkick::new("srs");
    ///
    /// // Perform an SRS wallkick on rotation failure
    /// for &(tx, ty) in wallkick.test(&block, &field, Rotation::R90) {
    ///     if block.rotate_at_offset(&field, Rotation::R90, (tx, ty)) {
    ///         break;
    ///     }
    /// }
    /// ```
    fn test(&self, block: &Block, field: &Field, r: Rotation) -> &'static [(i32, i32)];
}

macro_rules! gen_wallkick {
    ($wkid:ident) => {
        #[allow(missing_docs)]
        pub struct $wkid;

        static __INSTANCE: $wkid = $wkid {};

        impl $wkid {
            /// Returns a static instance to this wallkick.
            pub fn new() -> &'static $wkid {
                &__INSTANCE
            }
        }
    }
}

pub use self::srs::SRS;
pub use self::empty::Empty;
pub use self::simple::Simple;
pub use self::dtet::DTET;
pub use self::tgm::TGM;
pub use self::tgm3::TGM3;

mod srs;
mod empty;
mod simple;
mod dtet;
mod tgm;
mod tgm3;

/// Factory function for constructing rotation systems from name.
///
/// This is usually more convenient than explicit instantiation, as rotation
/// systems are often stored as a name only.
///
/// # Names
///  - `srs`
///  - `empty`
///  - `simple`
///  - `dtet`
///  - `tgm`
///  - `tgm3`
///
/// # Panics
///
/// `new` will panic if the input string is not one of the strings present in
/// `Names`.
pub fn new(name: &str) -> &'static Wallkick {
    match name {
        "srs" => SRS::new(),
        "empty" => Empty::new(),
        "simple" => Simple::new(),
        "dtet" => DTET::new(),
        "tgm" => TGM::new(),
        "tgm3" => TGM3::new(),
        _ => panic!("unknown wallkick")
    }
}
