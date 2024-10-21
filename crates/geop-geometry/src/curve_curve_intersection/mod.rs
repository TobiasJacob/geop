use crate::EQ_THRESHOLD;

// Alphabetical order
pub mod circle_circle;
pub mod circle_line;
pub mod curve_curve;
pub mod ellipse_ellipse;
pub mod helix_circle;
pub mod helix_line;
pub mod line_line;
pub mod numerical;

const PRECISION: f64 = EQ_THRESHOLD / 100.0;
