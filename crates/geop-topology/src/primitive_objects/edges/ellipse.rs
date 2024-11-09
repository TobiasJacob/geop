use geop_geometry::{
    curves::{curve::Curve, ellipse::Ellipse},
    point::Point,
};

use crate::topology::contour_no_point::ContourNoPoint;

pub fn primitive_ellipse(
    basis: Point,
    normal: Point,
    major_radius: Point,
    minor_radius: Point,
) -> ContourNoPoint {
    let e = Ellipse::new(basis, normal, major_radius, minor_radius);
    ContourNoPoint::new(Curve::Ellipse(e))
}
