use geop_geometry::{
    curves::{curve::Curve, helix::Helix},
    point::Point,
};

use crate::topology::contour_no_point::ContourNoPoint;

pub fn primitive_helix(
    basis: Point,
    pitch: Point,
    radius: Point,
    right_winding: bool,
) -> ContourNoPoint {
    let h = Helix::new(basis, pitch, radius, right_winding);
    ContourNoPoint::new(Curve::Helix(h))
}
