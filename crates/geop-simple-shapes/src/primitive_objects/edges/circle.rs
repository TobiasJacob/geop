use geop_geometry::{
    curves::{circle::Circle, curve::Curve},
    efloat::EFloat64,
    point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_circle(basis: Point, normal: Point, radius: EFloat64) -> Edge {
    let c = Circle::try_new(basis, normal.normalize().unwrap(), radius).unwrap();
    Edge::new(None, None, Curve::Circle(c))
}
