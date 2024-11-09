use geop_geometry::{
    curves::{circle::Circle, curve::Curve},
    efloat::EFloat64,
    point::Point,
};

use crate::topology::contour_no_point::ContourNoPoint;

pub fn primitive_circle(basis: Point, normal: Point, radius: EFloat64) -> ContourNoPoint {
    let c = Circle::new(basis, normal.normalize().unwrap(), radius);
    ContourNoPoint::new(Curve::Circle(c))
}
