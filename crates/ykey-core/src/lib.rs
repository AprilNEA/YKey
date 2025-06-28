// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

// Copyright 2025 AprilNEA LLC
// SPDX-License-Identifier: MIT

//! Core library for YKey hardware security key management

pub mod error;
pub mod types;
pub mod traits;

// Re-export commonly used types and traits
pub use error::{YKeyError, YKeyResult};
pub use traits::*;
pub use types::*;


