use geop_geometry::{curves::CurveLike, point::Point};

use crate::topology::contour::Contour;

use super::edge_point::{edge_point_contains, EdgePointContains};

pub fn contour_point_contains(contour: &Contour, point: Point) -> EdgePointContains {
    match contour {
        Contour::Curve(curve_contour) => match curve_contour.curve.on_curve(point) {
            true => {
                return EdgePointContains::Inside;
            }
            false => {
                return EdgePointContains::Outside;
            }
        },
        Contour::ConnectedEdge(composite_contour) => {
            for edge in composite_contour.edges.iter() {
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
        }
    }
    return EdgePointContains::Outside;
}
