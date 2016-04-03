//! This modules provides an interface for dealing with different block offsets.
//!
//! Offsets refer to particular rotation specifications. For example, the SRS
//! and Akira style rotation systems which each contain different offset values
//! which can both be used if they implement the RotationSystem trait.
//!
//! All `RotationsSystem`'s currently use an empty struct which can be passed
//! around. This allows generic usage of anything which impl's `RotationSystem`.

use block::Type;
use Rotation;

/// The `RotationSystem` trait is implmented by all rotation systems.
pub trait RotationSystem {

    /// Returns a static array of offset values for the specified `Type`
    /// and `Rotation`.
    fn data(&self, ty: Type, rotation: Rotation) -> &'static [(usize, usize)];
}

pub use self::srs::SRS;
pub mod srs;
