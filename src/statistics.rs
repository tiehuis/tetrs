//! Stores statistics about the current game.

/// Stores statistics about the current game.
#[derive(Default)]
pub struct Statistics {
    /// How many lines have been cleared
    pub lines: u64,

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
    pub fn new() -> Statistics {
        Statistics { ..Default::default() }
    }
}
