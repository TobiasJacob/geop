use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgeContains {
    Inside,
    Outside,
    OnPoint(Rc<Point>),
}

pub fn edge_contains_point(edge: &Edge, point: Point) -> EdgeContains {
    if !edge.curve.on_manifold(point) {
        return EdgeContains::Outside;
    }
    if point == *edge.start || point == *edge.end {
        return EdgeContains::OnPoint(Rc::new(point));
    }
    if edge.curve.between(point, *edge.start, *edge.end) {
        return EdgeContains::Inside;
    }
    EdgeContains::Outside
}
