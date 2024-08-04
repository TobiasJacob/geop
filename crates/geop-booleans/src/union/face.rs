use crate::remesh::face::{face_remesh, face_split, normalize_faces, FaceSplit};
use geop_topology::topology::face::Face;

pub fn face_face_union(face_self: &Face, face_other: &Face) -> Vec<Face> {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            FaceSplit::AinB(_) => false,
            FaceSplit::AonBSameSide(_) => true,
            FaceSplit::AonBOpSide(_) => false,
            FaceSplit::AoutB(_) => true,
            FaceSplit::BinA(_) => false,
            FaceSplit::BonASameSide(_) => false,
            FaceSplit::BonAOpSide(_) => false,
            FaceSplit::BoutA(_) => true,
        })
        .collect::<Vec<FaceSplit>>();

    let contours = face_remesh(edges);
    return normalize_faces(contours, face_self.surface.clone());
}
