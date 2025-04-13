pub const HORIZON_DIST: f64 = 100.0; // A big number to represent the distance to the horizon. Used only for visualization purposes.

pub mod algebra_error;
pub mod bounding_box;
pub mod color;
pub mod contour;
pub mod coordinate_system;
pub mod curves;
pub mod edge;
pub mod efloat;
pub mod face;
pub mod factorial;
pub mod geometry_error;
pub mod geometry_scene;
pub mod point;
pub mod primitives;
pub mod scene;
pub mod shell;
pub mod surfaces;
pub mod transforms;
pub mod volume;

pub mod topology_error;
pub mod topology_scene;

use efloat::EFloat64;
use point::Point;

pub trait MultiDimensionFunction {
    fn eval(&self, t: EFloat64) -> Point;
}
