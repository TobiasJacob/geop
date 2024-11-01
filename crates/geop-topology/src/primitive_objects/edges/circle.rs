use geop_geometry::{
    curves::{circle::Circle, curve::Curve},
    point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_circle(basis: Point, normal: Point, radius: f64) -> Edge {
    let c = Circle::new(basis, normal.normalize().unwrap(), radius);
    Edge::new(None, None, Curve::Circle(c))
}
