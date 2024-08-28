use geop_geometry::{
    curves::{curve::Curve, helix::Helix},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_helix(basis: Point, pitch: Point, radius: Point, right_winding: bool) -> Edge {
    let h = Helix::new(basis, pitch, radius, right_winding);
    Edge::new(None, None, Curve::Helix(h))
}
