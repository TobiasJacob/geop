use geop_geometry::{
    curves::{curve::Curve, ellipse::Ellipse},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_ellipse(
    basis: Point,
    normal: Point,
    major_radius: Point,
    minor_radius: Point,
) -> Edge {
    let e = Ellipse::new(basis, normal, major_radius, minor_radius);
    Edge::new(None, None, Curve::Ellipse(e))
}
