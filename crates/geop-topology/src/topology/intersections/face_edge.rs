use std::rc::Rc;

use crate::topology::{edge::Edge, contains::{face_point::{face_contains_point, FaceContainsPoint}, face_edge::{FaceContainsEdge, face_contains_edge}}, face::Face};

use super::edge_edge::EdgeEdgeIntersection;

pub fn face_edge_intersection(face: &Face, edge: &Edge) -> Vec<EdgeEdgeIntersection> {
    let mut intersections = face.surface.intersect_edge(edge);

    let mut new_interesections = Vec::<EdgeEdgeIntersection>::new();
    for int in intersections.drain(..) {
        match &int {
            EdgeEdgeIntersection::Point(p) => {
                match face_contains_point(face, **p) {
                    FaceContainsPoint::Inside => { new_interesections.push(int) },
                    _ => {}
                }
            },
            EdgeEdgeIntersection::Edge(e) => {
                let mut edges = vec![Rc::new(e.clone())];
                for b in face.boundaries.iter() {
                    // let ints = b.intersect_edge(&e);
                    edges = todo!("Split edges by intersections")
                    // edges = b.split_edges_if_necessary(edges);
                }

                for e in edges.drain(..) {
                    match face_contains_edge(face, &e) {
                        FaceContainsEdge::Inside => { new_interesections.push(EdgeEdgeIntersection::Edge((*e).clone())) },
                        FaceContainsEdge::OnBorderOppositeDir => { new_interesections.push(EdgeEdgeIntersection::Edge((*e).clone())) },
                        FaceContainsEdge::OnBorderSameDir => { new_interesections.push(EdgeEdgeIntersection::Edge((*e).clone())) },
                        FaceContainsEdge::Outside => {}
                    }
                }
            }
        }
    }

    intersections
}
