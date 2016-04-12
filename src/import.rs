//! Convenience module providing default imports.
//!
//! This imports all structs into the current scope, and imports
//! all other functions/enums namespaced by their base module.
//!
//! ## Examples
//!
//! ```
//! use tetrs::import::*;
//!
//! // Engine and Field are in scope
//! let engine = Engine::new(EngineOptions { ..Default::default() });
//! let field = Field::new();
//! ```
//!
//! This would otherwise require the following import.
//!
//! ```
//! use tetrs::engine::{Engine, EngineOptions};
//! use tetrs::field::Field;
//! ```

pub use engine::{Engine, EngineOptions, EngineSettings};
pub use block::{self, Block, BlockOptions, Rotation, Direction};
pub use field::{Field, FieldOptions};
pub use controller;
pub use randomizer::{self, Randomizer};
pub use wallkick::{self, Wallkick};
pub use rotation_system::{self, RotationSystem};
