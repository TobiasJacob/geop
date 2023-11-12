use crate::topology::{
    face::Face,
    remesh::face::{face_split, FaceSplit, face_remesh}, edge::Edge,
};

pub enum FaceFaceIntersection {
    Face(Face),
    Edge(Edge),
}

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
