use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{face::Face, edge::Edge, contains::{contour_point::contour_contains_point, edge_point::{EdgeContains, edge_contains_point}}};

use super::edge_edge::{EdgeEdgeIntersection, edge_edge_intersections};

pub fn countour_edge_intersection_points(face: &Face, edge: &Edge) -> Vec<Rc<Point>> {
    let mut intersections = Vec::<Rc<Point>>::new();
    for contour in face.boundaries.iter() {
        for edge_o in contour.edges.iter() {
            for int in edge_edge_intersections(edge, &edge_o) {
                match int {
                    EdgeEdgeIntersection::Point(point) => {
                        assert!(edge_contains_point(edge, *point) != EdgeContains::Outside, "edge: {:}, edge_o: {:}, point: {:?}", edge, edge_o, point);
                        assert!(edge_contains_point(edge_o, *point) != EdgeContains::Outside, "edge: {:}, edge_o: {:}, point: {:?}", edge, edge_o, point);
                        intersections.push(point);
                    }
                    EdgeEdgeIntersection::Edge(_edge) => {
                        assert!(edge_contains_point(edge_o, *_edge.start) != EdgeContains::Outside, "edge: {:}, edge_o: {:}, point: {:?}", edge, edge_o, _edge.start);
                        assert!(edge_contains_point(edge_o, *_edge.end) != EdgeContains::Outside, "edge: {:}, edge_o: {:}, point: {:?}", edge, edge_o, _edge.end);
                        assert!(edge_contains_point(edge, *_edge.start) != EdgeContains::Outside, "edge: {:}, edge_o: {:}, point: {:?}", edge, edge_o, _edge.start);
                        assert!(edge_contains_point(edge, *_edge.end) != EdgeContains::Outside, "edge: {:}, edge_o: {:}, point: {:?}", edge, edge_o, _edge.end);
                        intersections.push(_edge.start);
                        intersections.push(_edge.end);
                    }
                }
            }
        }
    }

    for int in intersections.iter() {
        let mut on_edge = false;
        for contour in face.boundaries.iter() {
            on_edge |= contour_contains_point(contour.clone(), **int) != EdgeContains::Outside;
        }
        assert!(on_edge);
    }

    intersections
}