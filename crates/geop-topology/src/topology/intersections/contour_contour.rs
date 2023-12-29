use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::face::Face;

use super::edge_edge::{edge_edge_intersections, EdgeEdgeIntersection};

pub fn countour_contour_intersection_points(face_self: &Face, face_other: &Face) -> Vec<Rc<Point>> {
    let mut intersections = Vec::<Rc<Point>>::new();
    for es in face_self.all_edges().iter() {
        for eo in face_other.all_edges().iter() {
            for int in edge_edge_intersections(&es, &eo) {
                match int {
                    EdgeEdgeIntersection::Point(point) => {
                        intersections.push(point);
                    }
                    EdgeEdgeIntersection::Edge(edge) => {
                        intersections.push(edge.start);
                        intersections.push(edge.end);
                    }
                }
            }
        }
    }

    intersections
}
