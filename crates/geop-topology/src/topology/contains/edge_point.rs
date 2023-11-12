use std::rc::Rc;

use geop_geometry::{points::point::Point, EQ_THRESHOLD};

use crate::topology::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeContains {
    Inside,
    Outside,
    OnPoint(Rc<Point>),
}


pub fn edge_contains_point(edge: &Edge, point: Point) -> EdgeContains {
    let u = edge.project(point);
    match u {
        Some(u) => {
            if u < EQ_THRESHOLD {
                EdgeContains::OnPoint(edge.start.clone())
            } else if u > 1.0 - EQ_THRESHOLD {
                EdgeContains::OnPoint(edge.end.clone())
            } else {
                EdgeContains::Inside
            }
        }
        None => EdgeContains::Outside,
    }
}