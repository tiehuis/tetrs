//! Stores statistics about an individual game.

/// `Statistics` is a 'dumb' struct, and does not provide any methods
/// upon it. Its primary use is as a namespacing tool to avoid
/// over-complicating struct such as `Engine`.
#[derive(Default)]
pub struct Statistics {
    /// How many lines have been cleared
    pub lines: u64,

    /// How many pieces have been placed
    pub pieces: u64,

    /// Total single line clears
    pub singles: u64,

    /// Total double line clears
    pub doubles: u64,

    /// Total triple line clears
    pub triples: u64,

    /// Total tetrises
    pub fours: u64,
}

impl Statistics {
    /// Construct a new `Statistics` object.
    ///
    /// Values are zeroed.
    pub fn new() -> Statistics {
        Statistics { ..Default::default() }
    }
}
