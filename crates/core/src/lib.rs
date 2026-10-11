//! Deterministic static geometry search, independent of GUI and persistence.
mod models;
mod search;
mod validation;

pub use models::*;
pub use search::{SearchControl, SearchError, SearchLimits, SearchReport, optimize};
pub use validation::validate;
