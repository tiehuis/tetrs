//! Stores statistics about the current game.

/// Stores statistics about the current game.
#[derive(Default)]
pub struct Statistics {
    /// How many lines have been cleared
    pub lines: u32,

    /// Total single line clears
    pub singles: u32,

    /// Total double line clears
    pub doubles: u32,

    /// Total triple line clears
    pub triples: u32,

    /// Total tetrises
    pub fours: u32,
}

impl Statistics {
    /// Construct a new `Statistics` object.
    pub fn new() -> Statistics {
        Statistics { ..Default::default() }
    }
}
