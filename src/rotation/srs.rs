//! Specifies the block values for the SRS rotation system.

use collections::enum_set::CLike;
use block::Type;
use Rotation;
use rotation::RotationSystem;

static I: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (3, 1)],
    [(2, 0), (2, 1), (2, 2), (2, 3)],
    [(0, 2), (1, 2), (2, 2), (3, 2)],
    [(1, 0), (1, 1), (1, 2), (1, 3)],
];

static T: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 0), (1, 1), (2, 1)],
    [(1, 0), (1, 1), (1, 2), (2, 1)],
    [(0, 1), (1, 1), (1, 2), (2, 1)],
    [(0, 1), (1, 0), (1, 1), (1, 2)],
];

static L: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 1), (2, 0), (2, 1)],
    [(1, 0), (1, 1), (1, 2), (2, 2)],
    [(0, 1), (0, 2), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (1, 1), (1, 2)],
];

static J: [[(usize, usize); 4]; 4] = [
    [(0, 0), (0, 1), (1, 1), (2, 1)],
    [(1, 0), (1, 1), (1, 2), (2, 0)],
    [(0, 1), (1, 1), (2, 1), (2, 2)],
    [(0, 2), (1, 0), (1, 1), (1, 2)],
];

static S: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 0), (1, 1), (2, 0)],
    [(1, 0), (1, 1), (2, 1), (2, 2)],
    [(0, 2), (1, 1), (1, 2), (2, 1)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
];

static Z: [[(usize, usize); 4]; 4] = [
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(1, 1), (1, 2), (2, 0), (2, 1)],
    [(0, 1), (1, 1), (1, 2), (2, 2)],
    [(0, 1), (0, 2), (1, 0), (1, 1)],
];

static O: [[(usize, usize); 4]; 4] = [
    [(1, 0), (1, 1), (2, 0), (2, 1)],
    [(1, 0), (1, 1), (2, 0), (2, 1)],
    [(1, 0), (1, 1), (2, 0), (2, 1)],
    [(1, 0), (1, 1), (2, 0), (2, 1)],
];

/// A struct containing SRS rotation data.
#[derive(Clone, Debug, Hash)]
pub struct SRS;

impl RotationSystem for SRS {
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

    rs_gen!();
}
