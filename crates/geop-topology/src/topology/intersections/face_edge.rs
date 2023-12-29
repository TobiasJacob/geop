use std::rc::Rc;

use crate::topology::{
    contains::{
        face_edge::{face_edge_contains, FaceEdgeContains},
        face_point::{face_point_contains, FacePointContains},
    },
    edge::Edge,
    face::Face,
};

use super::{edge_edge::EdgeEdgeIntersection, surface_edge::surface_edge_intersection};

pub fn face_edge_intersection(face: &Face, edge: &Edge) -> Vec<EdgeEdgeIntersection> {
    let mut intersections = surface_edge_intersection(&face.surface, edge);

    let mut new_interesections = Vec::<EdgeEdgeIntersection>::new();
    for int in intersections.drain(..) {
        match &int {
            EdgeEdgeIntersection::Point(p) => match face_point_contains(face, **p) {
                FacePointContains::Inside => new_interesections.push(int),
                _ => {}
            },
            EdgeEdgeIntersection::Edge(e) => {
                let mut edges = vec![Rc::new(e.clone())];
                for _b in face.boundaries.iter() {
                    // let ints = b.intersect_edge(&e);
                    let _edges = todo!("Split edges by intersections");
                    // edges = b.split_edges_if_necessary(edges);
                }

                for e in edges.drain(..) {
                    match face_edge_contains(face, &e) {
                        FaceEdgeContains::Inside => {
                            new_interesections.push(EdgeEdgeIntersection::Edge((*e).clone()))
                        }
                        FaceEdgeContains::OnBorderOppositeDir => {
                            new_interesections.push(EdgeEdgeIntersection::Edge((*e).clone()))
                        }
                        FaceEdgeContains::OnBorderSameDir => {
                            new_interesections.push(EdgeEdgeIntersection::Edge((*e).clone()))
                        }
                        FaceEdgeContains::Outside => {}
                    }
                }
            }
        }
    }

    intersections
}
