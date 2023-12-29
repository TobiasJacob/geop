use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{
    contains::{
        contour_point::contour_point_contains,
        edge_point::{edge_point_contains, EdgePointContains},
    },
    edge::Edge,
    face::Face,
};

use super::edge_edge::{edge_edge_intersections, EdgeEdgeIntersection};

pub fn countour_edge_intersection_points(face: &Face, edge: &Edge) -> Vec<Rc<Point>> {
    let mut intersections = Vec::<Rc<Point>>::new();
    for contour in face.boundaries.iter() {
        for edge_o in contour.edges.iter() {
            for int in edge_edge_intersections(edge, &edge_o) {
                match int {
                    EdgeEdgeIntersection::Point(point) => {
                        assert!(
                            edge_point_contains(edge, *point) != EdgePointContains::Outside,
                            "edge: {:}, edge_o: {:}, point: {:?}",
                            edge,
                            edge_o,
                            point
                        );
                        assert!(
                            edge_point_contains(edge_o, *point) != EdgePointContains::Outside,
                            "edge: {:}, edge_o: {:}, point: {:?}",
                            edge,
                            edge_o,
                            point
                        );
                        intersections.push(point);
                    }
                    EdgeEdgeIntersection::Edge(_edge) => {
                        assert!(
                            edge_point_contains(edge_o, *_edge.start) != EdgePointContains::Outside,
                            "edge: {:}, edge_o: {:}, point: {:?}",
                            edge,
                            edge_o,
                            _edge.start
                        );
                        assert!(
                            edge_point_contains(edge_o, *_edge.end) != EdgePointContains::Outside,
                            "edge: {:}, edge_o: {:}, point: {:?}",
                            edge,
                            edge_o,
                            _edge.end
                        );
                        assert!(
                            edge_point_contains(edge, *_edge.start) != EdgePointContains::Outside,
                            "edge: {:}, edge_o: {:}, point: {:?}",
                            edge,
                            edge_o,
                            _edge.start
                        );
                        assert!(
                            edge_point_contains(edge, *_edge.end) != EdgePointContains::Outside,
                            "edge: {:}, edge_o: {:}, point: {:?}",
                            edge,
                            edge_o,
                            _edge.end
                        );
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
            on_edge |= contour_point_contains(contour.clone(), **int) != EdgePointContains::Outside;
        }
        assert!(on_edge);
    }

    intersections
}
