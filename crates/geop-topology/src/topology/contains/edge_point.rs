use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::edge::Edge;

#[derive(Clone, Debug, PartialEq)]
pub enum EdgePointContains {
    Inside(usize),
    Outside,
    OnPoint(Rc<Point>),
}

impl EdgePointContains {
    pub fn is_inside(&self) -> bool {
        match self {
            EdgePointContains::Inside(_) => true,
            EdgePointContains::Outside => false,
            EdgePointContains::OnPoint(_) => false,
        }
    }
}

pub fn edge_point_contains(edge: &Edge, point: &Rc<Point>) -> EdgePointContains {
    if !edge.curve.on_manifold(**point) {
        return EdgePointContains::Outside;
    }
    for (s, e) in edge.boundaries.iter() {
        if point == s {
            return EdgePointContains::OnPoint(s.clone());
        }
        if point == e {
            return EdgePointContains::OnPoint(e.clone());
        }
    }

    for (i, (s, e)) in edge.boundaries.iter().enumerate() {
        if edge.curve.between(**point, **s, **e) {
            return EdgePointContains::Inside(i);
        }
    }

    EdgePointContains::Outside
}
