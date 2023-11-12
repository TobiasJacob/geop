use crate::topology::{
    face::Face,
    remesh::face::{face_split, EdgeSplit, face_remesh},
};

pub fn face_face_same_surface_intersection(face_self: &Face, face_other: &Face) -> Face {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            EdgeSplit::AinB(_) => true,
            EdgeSplit::AonBSameSide(_) => true,
            EdgeSplit::AonBOpSide(_) => false,
            EdgeSplit::AoutB(_) => false,
            EdgeSplit::BinA(_) => true,
            EdgeSplit::BonASameSide(_) => false,
            EdgeSplit::BonAOpSide(_) => false,
            EdgeSplit::BoutA(_) => false,
        }).collect::<Vec<EdgeSplit>>();

    return face_remesh(face_self.surface.clone(), edges);
}
