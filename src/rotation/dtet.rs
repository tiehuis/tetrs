//! Rotation offsrts for the DTET rotation system.

rs_gen!(DTET);

static I: [[(usize, usize); 4]; 4] = [
    [(0, 2), (1, 2), (2, 2), (3, 2)],
    [(2, 0), (2, 1), (2, 2), (2, 3)],
    [(0, 2), (1, 2), (2, 2), (3, 2)],
    [(1, 0), (1, 1), (1, 2), (1, 3)],
];

static T: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (1, 2)],
    [(0, 1), (1, 0), (1, 1), (1, 2)],
    [(0, 2), (1, 1), (1, 2), (2, 2)],
    [(1, 0), (1, 1), (1, 2), (2, 1)],
];

static L: [[(usize, usize); 4]; 4] = [
    [(0, 1), (0, 2), (1, 1), (2, 1)],
    [(0, 0), (1, 0), (1, 1), (1, 2)],
    [(0, 2), (1, 2), (2, 1), (2, 2)],
    [(1, 0), (1, 1), (1, 2), (2, 2)],
];

static J: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (2, 2)],
    [(0, 2), (1, 0), (1, 1), (1, 2)],
    [(0, 1), (0, 2), (1, 2), (2, 2)],
    [(1, 0), (1, 1), (1, 2), (2, 0)],
];

static S: [[(usize, usize); 4]; 4] = [
    [(0, 2), (1, 1), (1, 2), (2, 1)],
    [(1, 0), (1, 1), (2, 1), (2, 2)],
    [(0, 2), (1, 1), (1, 2), (2, 1)],
    [(0, 0), (0, 1), (1, 1), (1, 2)],
];

static Z: [[(usize, usize); 4]; 4] = [
    [(0, 1), (1, 1), (1, 2), (2, 2)],
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