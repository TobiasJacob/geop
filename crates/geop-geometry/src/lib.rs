pub const EQ_THRESHOLD: f64 = 1e-7; // TODO: Make this 1e-12, but add a larger threshold for f32 rasterization cases.

pub mod curves;
pub mod points;
pub mod surfaces;

pub mod curve_curve_intersection;
pub mod transforms;
