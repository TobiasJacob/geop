use geop_geometry::point::Point;

use geop_topology::{
    contains::{
        contour_point::contour_point_contains,
        edge_point::{edge_point_contains, EdgePointContains},
    },
    topology::{contour::Contour, edge::Edge},
};

use crate::BooleanPrimitiveEdge;

pub fn split_edge_by_point_if_necessary(
    edge: Contour,
    points: &[Point],
) -> Vec<BooleanPrimitiveEdge> {
    match edge {
        Contour::ContourMultiPoint(contour) => match contour_point_contains(contour, point) {
            EdgePointContains::Inside => contour.insert_point(point),
            _ => contour,
        },
        BooleanPrimitiveEdge::ContourSinglePoint(contour) => {
            match contour_point_contains(contour, point) {
                EdgePointContains::Inside => contour.insert_point(point),
                _ => contour,
            }
        }
        BooleanPrimitiveEdge::Edge(edge) => match edge_point_contains(edge, point) {
            EdgePointContains::Inside => {}
            _ => vec![edge],
        },
    }
}

// This operation splits an edge by a list of points.
// If one part goes to infinity, it will be removed.
pub fn split_edge_by_points_if_necessary(
    edge: BooleanPrimitiveEdge,
    points: &[Point],
) -> Vec<BooleanPrimitiveEdge> {
    for p in points {
        result = match result {
            Contour::ContourMultiPoint(contour) => {
                let mut new_result = Vec::<Edge>::new();
                for edge in contour.edges.iter() {
                    if edge_point_contains(edge, *p) != EdgePointContains::Inside {
                        new_result.push(edge.clone());
                    } else {
                        match (edge.bounds.start.clone(), edge.end.clone()) {
                            (Some(start), Some(end)) => {
                                new_result.push(Edge::new(
                                    Some(start),
                                    Some(p.clone()),
                                    edge.curve.clone(),
                                ));
                                new_result.push(Edge::new(
                                    Some(p.clone()),
                                    Some(end),
                                    edge.curve.clone(),
                                ));
                            }
                            (Some(start), None) => {
                                new_result.push(Edge::new(
                                    Some(start),
                                    Some(p.clone()),
                                    edge.curve.clone(),
                                ));
                            }
                            (None, Some(end)) => {
                                new_result.push(Edge::new(
                                    Some(p.clone()),
                                    Some(end),
                                    edge.curve.clone(),
                                ));
                            }
                            (None, None) => {
                                new_result.push(Edge::new(
                                    None,
                                    Some(p.clone()),
                                    edge.curve.clone(),
                                ));
                                new_result.push(Edge::new(
                                    Some(p.clone()),
                                    None,
                                    edge.curve.clone(),
                                ));
                            }
                        }
                    }
                }
                return result;
            }
            Contour::ContourNoPoint(curve) => {}
        };
    }
    result
}

pub fn split_edges_by_points_if_necessary(
    edges: Vec<BooleanPrimitiveEdge>,
    points: &Vec<Point>,
) -> Vec<Edge> {
    let mut result = Vec::<Edge>::new();
    for edge in edges {
        result.extend(split_contour_by_points_if_necessary(&edge, &points));
    }
    result
}

pub fn split_contour_by_points_if_necessary(
    contour: BooleanPrimitiveEdge,
    points: &Vec<Point>,
) -> Contour {
    Contour::new(split_edges_by_points_if_necessary(contour.edges, points))
}

pub fn split_contours_by_points_if_necessary(
    contours: Vec<Contour>,
    points: &Vec<Point>,
) -> Vec<Contour> {
    let mut result = Vec::<Contour>::new();
    for contour in contours {
        result.push(split_contour_by_points_if_necessary(contour, points));
    }
    result
}
