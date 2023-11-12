use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{face::Face, edge::Edge};

use super::edge_edge::{EdgeEdgeIntersection, edge_edge_intersections};

pub fn countour_edge_intersection_points(face: &Face, edge: &Edge) -> Vec<Rc<Point>> {
    let mut intersections = Vec::<Rc<Point>>::new();
    for contour in face.boundaries.iter() {
        for edge_o in contour.edges.iter() {
            for int in edge_edge_intersections(edge, &edge_o) {
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