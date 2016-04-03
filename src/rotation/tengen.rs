//! Specifies the block values for the Tengen rotation system.

use collections::enum_set::CLike;
use block::Type;
use Rotation;

static I: [[(usize, usize); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(1, 0), (1, 1), (1, 2), (1, 3)],
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(1, 0), (1, 1), (1, 2), (1, 3)],
];

static T: [[(usize, usize); 4]; 4] = [
    [(0, 0), (1, 0), (1, 1), (2, 0)],
    [(0, 1), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (1, 0), (1, 1), (2, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 1)],
];

static L: [[(usize, usize); 4]; 4] = [
    [(0, 0), (0, 1), (1, 0), (2, 0)],
    [(0, 0), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (1, 1), (2, 0), (2, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 2)],
];

static J: [[(usize, usize); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (2, 1)],
    [(0, 2), (1, 0), (1, 1), (1, 2)],
    [(0, 0), (0, 1), (1, 1), (2, 1)],
    [(0, 0), (0, 1), (0, 2), (1, 0)],
];

static S: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 0), (1, 1), (2, 0)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
    [(0, 1), (1, 0), (1, 1), (2, 0)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
];

static Z: [[(usize, usize); 4]; 4] = [
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(0, 1), (0, 2), (1, 0), (1, 1)],
    [(0, 0), (1, 0), (1, 1), (2, 1)],
    [(0, 1), (0, 2), (1, 0), (1, 1)],
];

static O: [[(usize, usize); 4]; 4] = [
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],
    [(0, 0), (0, 1), (1, 0), (1, 1)],
];

pub struct Tengen;

impl RotationSystem for Tengen {
    fn data(&self, ty: Type, rotation: Rotation) -> &'static [(usize, usize)] {
        match ty {
            Type::I => I[rotation.to_usize()],
            Type::T => T[rotation.to_usize()],
            Type::L => L[rotation.to_usize()],
            Type::J => J[rotation.to_usize()],
            Type::S => S[rotation.to_usize()],
            Type::Z => Z[rotation.to_usize()],
            Type::O => O[rotation.to_usize()],
            _ => panic!("Attempted to get data for Type: {:?}", ty)
        }
    }
}
