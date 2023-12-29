use geop_geometry::points::point::Point;

use crate::topology::contour::Contour;

use super::edge_point::{edge_contains_point, EdgeContains};

pub fn contour_contains_point(contour: Contour, point: Point) -> EdgeContains {
    for edge in contour.edges.iter() {
        match edge_contains_point(&edge, point) {
            EdgeContains::Inside => return EdgeContains::Inside,
            EdgeContains::OnPoint(point) => return EdgeContains::OnPoint(point),
            EdgeContains::Outside => continue,
        }
    }
    EdgeContains::Outside
}
