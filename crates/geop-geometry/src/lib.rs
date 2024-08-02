pub const EQ_THRESHOLD: f64 = 1e-7; // TODO: Make this 1e-12, but add a larger threshold for f32 rasterization cases.
pub const HORIZON_DIST: f64 = 1e5; // A big number to represent the distance to the horizon.

pub mod curves;
pub mod points;
pub mod surfaces;

pub mod curve_curve_intersection;
pub mod curve_surface_intersection;
pub mod surface_surface_intersection;
pub mod transforms;
