use geop_geometry::{curves, points::point::Point};

use crate::topology::{
    face::{Face, face_surface::{FaceSurfaceIntersection, face_surface_face_surface_intersect}},
    remesh::face::{face_split, FaceSplit, face_remesh}, edge::Edge,
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
    Point(Point),
    Edge(Edge),
    Face(Face),
}

pub fn face_face_different_surface(face_self: &Face, face_other: &Face) -> Vec<FaceFaceIntersection> {
    assert!(
        face_self.surface != face_other.surface,
        "Faces must have different surfaces",
    );

    match face_surface_face_surface_intersect(&face_self.surface, &face_other.surface) {
        FaceSurfaceIntersection::None => vec![],
        FaceSurfaceIntersection::CurvesAndPoints(curves, points) => {
            todo!("FaceFaceIntersection::CurvesAndPoints")
        },
        FaceSurfaceIntersection::Surface(surface) => {
            vec![FaceFaceIntersection::Face(face_face_same_surface_intersection(face_self, face_other))]
        },
    }
}