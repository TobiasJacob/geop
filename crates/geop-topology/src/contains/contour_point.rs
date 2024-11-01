use geop_geometry::point::Point;

use crate::topology::contour::Contour;

use super::edge_point::{edge_point_contains, EdgePointContains};

pub fn contour_point_contains(contour: &Contour, point: Point) -> EdgePointContains {
    for edge in contour.edges.iter() {
        let contains: EdgePointContains = edge_point_contains(edge, point);
        match contains {
            EdgePointContains::OnPoint(point) => {
                return EdgePointContains::OnPoint(point);
            }
            EdgePointContains::Inside => {
                return EdgePointContains::Inside;
            }
            EdgePointContains::Outside => {}
        }
    }
    return EdgePointContains::Outside;
}
