use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::contour::Contour;

use super::edge_point::{edge_point_contains, EdgePointContains};

pub fn contour_point_contains(contour: &Contour, point: &Rc<Point>) -> EdgePointContains {
    for edge in contour.edges.iter() {
        match edge_point_contains(&edge, point) {
            EdgePointContains::Inside(i) => return EdgePointContains::Inside(i),
            EdgePointContains::OnPoint(point) => return EdgePointContains::OnPoint(point),
            EdgePointContains::Outside => continue,
        }
    }
    EdgePointContains::Outside
}
