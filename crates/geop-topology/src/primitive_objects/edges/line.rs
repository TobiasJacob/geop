use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_line(start: Point, end: Point) -> Edge {
    let l = Line::new(start, end - start);
    Edge::new(Some(start), Some(end), Curve::Line(l))
}

pub fn primitive_infinite_line(p1: Point, p2: Point) -> Edge {
    let l = Line::new(p1, p2 - p1);
    Edge::new(None, None, Curve::Line(l))
}
