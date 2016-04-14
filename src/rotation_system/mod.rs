//! This modules provides an interface for dealing with different block offsets.
//!
//! Offsets refer to particular rotation specifications. For example, the SRS
//! and Akira style rotation systems which each contain different offset values
//! which can both be used if they implement the `RotationSystem` trait.

use block::{Id, Rotation};
use std::cmp;

/// The `RotationSystem` trait is implmented by all rotation systems.
///
/// When implementing `RotationSystem`, the only thing that is required to
/// implement are the offset values for each of the main blocks
/// `I, J, L, S, Z, T, O`.
///
/// See the `srs.rs` file for an example.
pub trait RotationSystem {

    /// Returns a static array of offset values for the specified `Id`
    /// and `Rotation`.
    fn data(&self, ty: Id, rotation: Rotation) -> &'static [(usize, usize)];

    /// Returns the minimum offset of the first piece in a block.
    ///
    /// Return the offset from the `(x, y)` bounding coordinate to the first
    /// non-empty piece in a block. This row by row from `y = 0` onwards.
    ///
    /// ```
    /// use tetrs::import::*;
    ///
    /// // The piece marked '@' is the first encountered piece.
    /// // ...
    /// // @##
    /// // .#.
    ///
    /// let rs = rotation_system::new("srs");
    ///
    /// let (x1, y1) = rs.minp(block::Id::T, Rotation::R180);
    /// assert_eq!((x1, y1), (0, 1));
    ///
    /// ```
    fn minp(&self, id: Id, rotation: Rotation) -> (usize, usize) {
        self.data(id, rotation).iter()
            .fold((!0, !0), |(a, b), &(x, y)| {
                // We want the least-(x, y) such that y is minimized.
                // This is subtly different from offset which allows the
                // minimum of (x, y) from any multiple blocks.
                if y < b || (y == b && x <= a) {
                    (x, y)
                }
                else {
                    (a, b)
                }
            })
    }

    /// Returns a tuple containing the leading empty `(x, y)` columns.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let rs = rotation_system::new("srs");
    ///
    /// // An L-block can have the following representation
    /// // .#.
    /// // .#.
    /// // .##
    ///
    /// let (x1, y1) = rs.min(block::Id::L, Rotation::R90);
    /// assert_eq!((x1, y1), (1, 0));
    ///
    /// // An I-block can have the following representation
    /// // ....
    /// // ....
    /// // ####
    /// // ....
    ///
    /// let (x2, y2) = rs.min(block::Id::I, Rotation::R180);
    /// assert_eq!((x2, y2), (0, 2));
    ///
    /// ```
    fn min(&self, id: Id, rotation: Rotation) -> (usize, usize) {
        self.data(id, rotation).iter()
            .fold((!0, !0), |(a, b), &(x, y)| {
                (cmp::min(a, x), cmp::min(b, y))
            })
    }

    /// Returns an `(x, y)` tuple containing the maximum offsets for the
    /// specified block.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::import::*;
    ///
    /// let rs = rotation_system::new("srs");
    ///
    /// // An L-block can have the following representation
    /// // .#.
    /// // .#.
    /// // .##
    ///
    /// let (x1, y1) = rs.max(block::Id::L, Rotation::R90);
    /// assert_eq!((x1, y1), (2, 2));
    ///
    /// // An I-block can have the following representation
    /// // ....
    /// // ....
    /// // ####
    /// // ....
    ///
    /// let (x2, y2) = rs.max(block::Id::I, Rotation::R180);
    /// assert_eq!((x2, y2), (3, 2));
    ///
    /// ```
    fn max(&self, id: Id, rotation: Rotation) -> (usize, usize) {
        self.data(id, rotation).iter()
            .fold((0, 0), |(a, b), &(x, y)| {
                (cmp::max(a, x), cmp::max(b, y))
            })
    }
}

/// Generates all data fields for a `RotationSystem`. The only requirement is
/// to implement the block offsets in static arrays.
///
/// This could work as a derive attribute probably, but that is extra work.
macro_rules! rs_gen {
    ($id:ident) => {
        use collections::enum_set::CLike;
        use block::{Id, Rotation};
        use rotation_system::RotationSystem;

        #[allow(missing_docs)]
        pub struct $id;

        // Each module gets its own static instance it can use
        static __INSTANCE: $id = $id { };

        impl $id {
            /// Return a new instance
            pub fn new() -> &'static $id {
                &__INSTANCE
            }
        }

        impl RotationSystem for $id {
            fn data(&self, ty: Id, rotation: Rotation) -> &'static [(usize, usize)] {
                match ty {
                    Id::I => &I[rotation.to_usize()],
                    Id::T => &T[rotation.to_usize()],
                    Id::L => &L[rotation.to_usize()],
                    Id::J => &J[rotation.to_usize()],
                    Id::S => &S[rotation.to_usize()],
                    Id::Z => &Z[rotation.to_usize()],
                    Id::O => &O[rotation.to_usize()],
                    _ => panic!("Attempted to get data for Id: {:?}", ty)
                }
            }
        }
    }
}

pub use self::srs::SRS;
pub use self::ars::ARS;
pub use self::tengen::Tengen;
pub use self::dtet::DTET;

pub mod srs;
pub mod ars;
pub mod tengen;
pub mod dtet;

/// Factory function for constructing a rotation system from name.
///
/// A rotation system is usually stored as a string and is much easier
/// to construct based on this thantheir direct `new` functions.
///
/// # Names
///  - `srs`
///  - `dtet`
///  - `arika`
///  - `tengen`
///
/// # Panics
///
/// `new` will panic if the input string is not one of the strings present in
/// `Names`.
pub fn new(name: &str) -> &'static RotationSystem {
    match name {
        "srs" => SRS::new(),
        "dtet" => DTET::new(),
        "ars" => ARS::new(),
        "tengen" => Tengen::new(),
        _ => panic!("unknown rotation system: {}", name)
    }
}

// If we can guarantee `max`, `min`, and `minp` from `rs_gen!()` work, then we
// only require testing for one case (`SRS` in this example).
#[cfg(test)]
mod tests {
    use ::import::*;
    use ::block::Id;

    #[test]
    fn test_offset_to_first1() {
        let rs = rotation_system::SRS::new();

        assert_eq!((1, 0), rs.minp(Id::T, Rotation::R0));
        assert_eq!((1, 0), rs.minp(Id::T, Rotation::R90));
        assert_eq!((0, 1), rs.minp(Id::T, Rotation::R180));
        assert_eq!((1, 0), rs.minp(Id::T, Rotation::R270));

        assert_eq!((2, 0), rs.minp(Id::I, Rotation::R90));
        assert_eq!((0, 0), rs.minp(Id::Z, Rotation::R0));
    }
}
