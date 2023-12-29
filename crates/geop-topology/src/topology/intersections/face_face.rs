use std::{rc::Rc, f32::consts::E};

use geop_geometry::{curves, points::point::Point};

use crate::topology::{
    face::{Face, face_surface::{FaceSurfaceIntersection, face_surface_face_surface_intersect}},
    remesh::face::{face_split, FaceSplit, face_remesh}, edge::Edge, contains::{face_point::{face_contains_point, FaceContainsPoint}, face_edge}, intersections::{face_edge::face_edge_intersection, edge_edge::EdgeEdgeIntersection},
};


pub fn face_face_same_surface_intersection(face_self: &Face, face_other: &Face) -> Face {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            FaceSplit::AinB(_) => true,
            FaceSplit::AonBSameSide(_) => true,
            FaceSplit::AonBOpSide(_) => false,
            FaceSplit::AoutB(_) => false,
            FaceSplit::BinA(_) => true,
            FaceSplit::BonASameSide(_) => false,
            FaceSplit::BonAOpSide(_) => false,
            FaceSplit::BoutA(_) => false,
        }).collect::<Vec<FaceSplit>>();

    return face_remesh(face_self.surface.clone(), edges);
}

pub enum FaceFaceIntersection {
    None,
    Point(Rc<Point>),
    Edge(Edge),
    Face(Face),
}

pub fn face_face_intersection(face_self: &Face, face_other: &Face) -> Vec<FaceFaceIntersection> {
    assert!(
        face_self.surface != face_other.surface,
        "Faces must have different surfaces",
    );

    match face_surface_face_surface_intersect(&face_self.surface, &face_other.surface) {
        FaceSurfaceIntersection::None => vec![],
        FaceSurfaceIntersection::CurvesAndPoints(curves, points) => {
            let mut intersections = vec![];
            for point in points.iter() {
                if face_contains_point(face_self, *point) != FaceContainsPoint::Outside {
                    if face_contains_point(face_other, *point) != FaceContainsPoint::Outside {
                        intersections.push(FaceFaceIntersection::Point(Rc::new(point.clone())));
                    }
                }
            }

            let mut segments: Vec<EdgeEdgeIntersection> = vec![];
            for curve in curves.iter() {
                let edge = Edge::new(todo!("Start"), todo!("End"), Rc::new(curve.clone()));
                segments.push(EdgeEdgeIntersection::Edge(edge));
            }

            for face in &[face_self, face_other] {
                let mut new_segments = vec![];

                for seg in segments.iter() {
                    match seg {
                        EdgeEdgeIntersection::Point(p) => {
                            if face_contains_point(face, **p) != FaceContainsPoint::Outside {
                                new_segments.push(seg.clone());
                            }
                        },
                        EdgeEdgeIntersection::Edge(e) => {
                            new_segments.extend(face_edge_intersection(face_self, e));
                        }
                    }
                }

                segments = new_segments;
            }

            segments.iter().map(|seg| {
                match seg {
                    EdgeEdgeIntersection::Point(p) => {
                        FaceFaceIntersection::Point(p.clone())
                    },
                    EdgeEdgeIntersection::Edge(e) => {
                        FaceFaceIntersection::Edge(e.clone())
                    }
                }
            }).collect()
        },
        FaceSurfaceIntersection::Surface(surface) => {
            vec![FaceFaceIntersection::Face(face_face_same_surface_intersection(face_self, face_other))]
        },
    }
}