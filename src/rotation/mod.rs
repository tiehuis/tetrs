//! This modules provides an interface for dealing with different block offsets.
//!
//! Offsets refer to particular rotation specifications. For example, the SRS
//! and Akira style rotation systems which each contain different offset values
//! which can both be used if they implement the `RotationSystem` trait.
//!
//! All `RotationsSystem`'s currently use an empty struct which can be passed
//! around. This allows generic usage of anything which impl's `RotationSystem`.

use block::{Type, Rotation};

/// The `RotationSystem` trait is implmented by all rotation systems.
///
/// When implementing `RotationSystem`, the only thing that is required to
/// implement are the offset values for each of the main blocks
/// `I, J, L, S, Z, T, O`.
///
/// See the `srs.rs` file for an example.
pub trait RotationSystem {

    /// Returns a static array of offset values for the specified `Type`
    /// and `Rotation`.
    fn data(&self, ty: Type, rotation: Rotation) -> &'static [(usize, usize)];

    /// Returns a tuple containing the leading empty `(x, y)` columns.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::block::{Rotation, Type};
    /// use tetrs::rotation::{self, RotationSystem};
    ///
    /// let rs = rotation::SRS{};
    ///
    /// // An L-block can have the following representation
    /// // .#.
    /// // .#.
    /// // .##
    ///
    /// let (x1, y1) = rs.min(Type::L, Rotation::R90);
    /// assert_eq!((x1, y1), (1, 0));
    ///
    /// // An I-block can have the following representation
    /// // ....
    /// // ....
    /// // ####
    /// // ....
    ///
    /// let (x2, y2) = rs.min(Type::I, Rotation::R180);
    /// assert_eq!((x2, y2), (0, 2));
    ///
    /// ```
    fn min(&self, id: Type, r: Rotation) -> (usize, usize) {
        use std::cmp;
        self.data(id, r).iter()
            .fold((!0, !0), |(a, b), &(x, y)| {
                (cmp::min(a, x), cmp::min(b, y))
            })
    }

    /// Returns an `(x, y)` tuple containing the maximum offsets for the
    /// specified block.
    ///
    /// ## Examples
    /// ```
    /// use tetrs::block::{Rotation, Type};
    /// use tetrs::rotation::{self, RotationSystem};
    ///
    /// let rs = rotation::SRS{};
    ///
    /// // An L-block can have the following representation
    /// // .#.
    /// // .#.
    /// // .##
    ///
    /// let (x1, y1) = rs.max(Type::L, Rotation::R90);
    /// assert_eq!((x1, y1), (2, 2));
    ///
    /// // An I-block can have the following representation
    /// // ....
    /// // ....
    /// // ####
    /// // ....
    ///
    /// let (x2, y2) = rs.max(Type::I, Rotation::R180);
    /// assert_eq!((x2, y2), (3, 2));
    ///
    /// ```
    fn max(&self, id: Type, r: Rotation) -> (usize, usize) {
        use std::cmp;
        self.data(id, r).iter()
            .fold((0, 0), |(a, b), &(x, y)| {
                (cmp::max(a, x), cmp::max(b, y))
            })
    }

    /// Returns the minimum offset of the first piece in a block.
    ///
    /// Return the offset from the `(x, y)` bounding coordinate to the first
    /// non-empty piece in a block. This row by row from `y = 0` onwards.
    ///
    /// ```
    /// use tetrs::block::{Rotation, Type};
    /// use tetrs::rotation;
    /// use tetrs::rotation::RotationSystem;
    ///
    /// // The piece marked '@' is the first encountered piece.
    /// // ...
    /// // @##
    /// // .#.
    ///
    /// let rs = rotation::SRS{};
    ///
    /// let (x1, y1) = rs.minp(Type::T, Rotation::R180);
    /// assert_eq!((x1, y1), (0, 1));
    ///
    /// ```
    fn minp(&self, id: Type, r: Rotation) -> (usize, usize) {
        self.data(id, r).iter()
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
}

/// Generates all data fields for a `RotationSystem`. The only requirement is
/// to implement the block offsets in static arrays.
///
/// This could work as a derive attribute probably, but that is extra work.
macro_rules! rs_gen {
    ($id:ident) => {
        use collections::enum_set::CLike;
        use block::{Type, Rotation};
        use rotation::RotationSystem;

        #[derive(Clone, Debug, Default, Hash)]
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
            fn data(&self, ty: Type, rotation: Rotation) -> &'static [(usize, usize)] {
                match ty {
                    Type::I => &I[rotation.to_usize()],
                    Type::T => &T[rotation.to_usize()],
                    Type::L => &L[rotation.to_usize()],
                    Type::J => &J[rotation.to_usize()],
                    Type::S => &S[rotation.to_usize()],
                    Type::Z => &Z[rotation.to_usize()],
                    Type::O => &O[rotation.to_usize()],
                    _ => panic!("Attempted to get data for Type: {:?}", ty)
                }
            }
        }
    }
}

pub use self::srs::SRS;
pub mod srs;

pub use self::arika::Arika;
pub mod arika;

pub use self::tengen::Tengen;
pub mod tengen;

pub use self::dtet::DTET;
pub mod dtet;

// If we can guarantee `max`, `min`, and `minp` from `rs_gen!()` work, then we
// only require testing for one case (`SRS` in this example).
#[cfg(test)]
mod tests {
    use super::*;
    use rotation;
    use block::{Rotation, Type};

    #[test]
    fn test_offset_to_first1() {
        let rs = rotation::SRS::new();

        assert_eq!((1, 0), rs.minp(Type::T, Rotation::R0));
        assert_eq!((1, 0), rs.minp(Type::T, Rotation::R90));
        assert_eq!((0, 1), rs.minp(Type::T, Rotation::R180));
        assert_eq!((1, 0), rs.minp(Type::T, Rotation::R270));

        assert_eq!((2, 0), rs.minp(Type::I, Rotation::R90));
        assert_eq!((0, 0), rs.minp(Type::Z, Rotation::R0));
    }
}
