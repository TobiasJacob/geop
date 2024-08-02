use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_line(start: Point, end: Point) -> Edge {
    let l = Line::new(start, end - start);
    Edge::new(Some(start), Some(end), Curve::Line(l))
}
