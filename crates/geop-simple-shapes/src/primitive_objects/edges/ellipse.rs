use geop_geometry::{
    curves::{curve::Curve, ellipse::Ellipse},
    point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_ellipse(
    basis: Point,
    normal: Point,
    major_radius: Point,
    minor_radius: Point,
) -> Edge {
    let e = Ellipse::try_new(basis, normal, major_radius, minor_radius).unwrap();
    Edge::new(None, None, Curve::Ellipse(e))
}
