use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgePointContains {
    Inside,
    Outside,
    OnPoint(Point),
}

pub fn edge_point_contains(edge: &Edge, point: Point) -> EdgePointContains {
    if !edge.curve.on_manifold(point) {
        return EdgePointContains::Outside;
    }
    if point == edge.start || point == edge.end {
        return EdgePointContains::OnPoint(point);
    }
    if edge.curve.between(point, edge.start, edge.end) {
        return EdgePointContains::Inside;
    }
    EdgePointContains::Outside
}
