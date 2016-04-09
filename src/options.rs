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

    /// ARE time (in ms)
    pub are: u64,

    /// Gravity (in ms). How many ms must pass for block to fall
    pub gravity: u64,

    /// Auto-repeat-rate (in ms)
    pub arr: u64,

    /// Is hold enabled
    pub has_hold: bool,

    /// How many times can we hold
    pub hold_limit: u64,

    /// Is hard drop allowed
    pub has_hard_drop: bool,

    /// Has soft drop
    pub has_soft_drop: bool,

    /// The speed soft drop works
    pub soft_drop_speed: u64,

    /// Maximum number of preview pieces
    pub preview_count: u64
}

impl Default for Options {
    fn default() -> Options {
        Options {
            das: 180,
            are: 0,
            gravity: 1000,
            arr: 16,
            has_hold: true,
            hold_limit: 1,
            has_hard_drop: true,
            has_soft_drop: true,
            soft_drop_speed: 16, // 1G
            preview_count: 3
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
