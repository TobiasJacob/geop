use geop_geometry::points::point::Point;

use geop_topology::{
    contains::edge_point::{edge_point_contains, EdgePointContains},
    topology::{contour::Contour, edge::Edge},
};

pub fn split_edge_by_points_if_necessary(edge: &Edge, points: &[Point]) -> Vec<Edge> {
    let mut result = vec![edge.clone()];
    for p in points {
        let mut new_result = Vec::<Edge>::new();
        for edge in result.iter() {
            if edge_point_contains(edge, *p) != EdgePointContains::Inside {
                new_result.push(edge.clone());
            } else {
                new_result.push(Edge::new(
                    edge.start.clone(),
                    Some(p.clone()),
                    edge.curve.clone(),
                ));
                new_result.push(Edge::new(
                    Some(p.clone()),
                    edge.end.clone(),
                    edge.curve.clone(),
                ));
            }
        }
        result = new_result;
    }
    result
}

pub fn split_edges_by_points_if_necessary(edges: Vec<Edge>, points: &Vec<Point>) -> Vec<Edge> {
    let mut result = Vec::<Edge>::new();
    for edge in edges {
        result.extend(split_edge_by_points_if_necessary(&edge, &points));
    }
    result
}

pub fn split_contour_by_points_if_necessary(contour: Contour, points: &Vec<Point>) -> Contour {
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
