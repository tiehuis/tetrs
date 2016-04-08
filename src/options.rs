//! Defines immutable options which are used by an engine.
//!
//! Something is an option if it is immutable. Also, consider the difference
//! between a `Player` option, and a `Mode` option.

/// Stores a number of internal options that may be useful during a games
/// execution.
///
/// Unlike `NullpoMino` these are never tied to a name. This can be handled by
/// the caller if required.
///
/// Currently `Options` do not manage the randomizer/rotation system etc. Need
/// to determine exactly what is required.
pub struct Options {
    /// DAS setting (in ms)
    pub das: u64,

    /// How many preview pieces are displayed
    pub preview_count: usize,

    /// Should ghost be displayed
    pub ghost_enabled: bool
}

impl Default for Options {
    fn default() -> Options {
        Options {
            das: 150,
            preview_count: 4,
            ghost_enabled: true
        }
    }
}

impl Options {
    /// Construct a new `Options` value.
    ///
    /// If more specific options are required, make use of the `Default`
    /// trait impl.
    ///
    /// ```ignore
    /// use options::Options;
    ///
    /// // Standard initialization
    /// let options = Options::new();
    ///
    /// // Specific initialization overriding only the `das` field
    /// let options = Options {
    ///     das: 110,
    ///     ..Default::default()
    /// }
    /// ```
    pub fn new() -> Options {
        Options { ..Default::default() }
    }
}
