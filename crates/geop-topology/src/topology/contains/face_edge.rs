use geop_geometry::points::point::Point;

use crate::topology::{face::Face, edge::Edge, intersections::edge_edge::EdgeEdgeIntersection, contains::face_point::{FaceContainsPoint, face_contains_point}};



pub enum FaceContainsEdge {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}



// Checks if an edge is inside the face. This guarantees that the edge is not touching any curves. The start and end point of the edge can be on the border, since they are not considered a part of the edge.
pub fn face_contains_edge(face: &Face, edge: &Edge) -> FaceContainsEdge {
    assert!(face.surface.contains_edge(edge));
    let mut intersections = Vec::<Point>::new();
    for contour in face.boundaries.iter() {
        let intersection = contour.intersect_edge(edge);
        for int in intersection {
            match int {
                EdgeEdgeIntersection::Point(point) => intersections.push(*point),
                EdgeEdgeIntersection::Edge(edge) => {
                    intersections.push(*edge.start);
                    intersections.push(*edge.end);
                }
            }
        }
    }

    let mut part_inside = false;
    let mut part_outside = false;
    for i in -1..intersections.len() as isize {
        let prev = if i == -1 {
            &edge.start
        } else {
            &intersections[i as usize]
        };
        let next = if i == intersections.len() as isize - 1 {
            &edge.end
        } else {
            &intersections[(i + 1) as usize]
        };
        let p = edge.get_midpoint(*prev, *next);
        match face_contains_point(face, p) {
            FaceContainsPoint::Inside => part_inside = true,
            FaceContainsPoint::Outside => part_outside = true,
            FaceContainsPoint::OnEdge(_) => (),
            FaceContainsPoint::OnPoint(_) => (),
        }
    }

    let p = edge.get_midpoint(*edge.start, *edge.end);

    match (part_inside, part_outside) {
        (true, true) => panic!("Edge is wiggleing on border"),
        (true, false) => FaceContainsEdge::Inside,
        (false, true) => FaceContainsEdge::Outside,
        (false, false) => match face.boundary_tangent(p).expect_on_edge().dot(edge.tangent(p)) > 0.0 {
            true => FaceContainsEdge::OnBorderSameDir,
            false => FaceContainsEdge::OnBorderOppositeDir,
        },
    }
}