use geop_geometry::{
    curves::{curve::Curve, helix::Helix},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_helix(basis: Point, pitch: Point, radius: Point) -> Edge {
    let h = Helix::new(basis, pitch, radius);
    Edge::new(None, None, Curve::Helix(h))
}
