use crate::topology::{face::Face, remesh::face::{EdgeSplit, face_split, face_remesh}};


pub fn face_union(face_self: &Face, face_other: &Face) -> Face {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            EdgeSplit::AinB(_) => false,
            EdgeSplit::AonBSameSide(_) => true,
            EdgeSplit::AonBOpSide(_) => false,
            EdgeSplit::AoutB(_) => true,
            EdgeSplit::BinA(_) => false,
            EdgeSplit::BonASameSide(_) => false,
            EdgeSplit::BonAOpSide(_) => false,
            EdgeSplit::BoutA(_) => true,
        }).collect::<Vec<EdgeSplit>>();

    return face_remesh(face_self.surface.clone(), edges);
}
