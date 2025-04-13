use crate::{curves::CurveLike, point::Point};

use crate::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgePointContains {
    Inside,
    Outside,
    OnPoint(Point),
}

pub fn edge_point_contains(edge: &Edge, point: Point) -> EdgePointContains {
    if !edge.curve.on_curve(point) {
        return EdgePointContains::Outside;
    }
    if Some(point) == edge.start || Some(point) == edge.end {
        return EdgePointContains::OnPoint(point);
    }
    if edge.curve.between(point, edge.start, edge.end).unwrap() {
        return EdgePointContains::Inside;
    }
    EdgePointContains::Outside
}
