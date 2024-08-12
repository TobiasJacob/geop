use geop_geometry::{
    curves::{curve::Curve, ellipsis::Ellipsis},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_ellipsis(
    basis: Point,
    normal: Point,
    major_radius: Point,
    minor_radius: Point,
) -> Edge {
    let e = Ellipsis::new(basis, normal, major_radius, minor_radius);
    Edge::new(None, None, Curve::Ellipsis(e))
}
