//! Deterministic static geometry search, independent of GUI and persistence.
mod models;
mod search;
mod validation;

pub use models::*;
pub use search::{SearchControl, SearchError, SearchLimits, SearchReport, optimize};
pub use validation::validate;
mod geometry;
pub use geometry::{Geometry, GeometryRect, schematic};

mod head;
pub use head::{
    HeadDiagramDimensions, HeadFormat, HeadGeometry, HeadGeometryError, HeadPosition,
    final_head_geometry, head_diagram_dimensions,
};

mod assignments;

mod dimensions;
pub use dimensions::{CandidateDimensions, PocketGap, candidate_dimensions};
