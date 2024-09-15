pub const EQ_THRESHOLD: f64 = 1e-7; // TODO: Make this 1e-12, but add a larger threshold for f32 rasterization cases.
pub const HORIZON_DIST: f64 = 100.0; // A big number to represent the distance to the horizon. Used only for visualization purposes.

pub mod bounding_box;
pub mod curve_curve_intersection;
pub mod curve_surface_intersection;
pub mod curves;
pub mod efloat;
pub mod points;
pub mod surface_surface_intersection;
pub mod surfaces;
pub mod transforms;
