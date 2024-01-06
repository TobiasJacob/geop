use crate::topology::{
    contains::{
        face_point::{face_point_contains, FacePointContains},
    },
    edge::Edge,
    face::Face, volume::Volume,
};

use super::volume_point::{volume_point_contains, VolumePointContains};

pub enum VolumeFaceContains {
    Inside,
    OnBorderSameDir,
    OnBorderOppositeDir,
    Outside,
}

// Checks if a face is inside a volume. The face is not allowed to wiggle inside and outside the volume, but has to stay on one side of the volume boundary faces.
pub fn volume_face_contains(volume: &Volume, face: &Face) -> VolumeFaceContains {
    // TODO: Make an assertian that there are no intersections with the volume boundaries

    let p = face.get_midpoint();
    match volume_point_contains(volume, p) {
        VolumePointContains::Inside => VolumeFaceContains::Inside,
        VolumePointContains::Outside => VolumeFaceContains::Outside,
        VolumePointContains::OnFace(_) => match face
            .normal(p)
            .dot(face.normal(p))
            > 0.0
        {
            true => VolumeFaceContains::OnBorderSameDir,
            false => VolumeFaceContains::OnBorderOppositeDir,
        },
        VolumePointContains::OnEdge(_) => panic!("This case should not happen"),
        VolumePointContains::OnPoint(_) => panic!("This case should not happen"),
    }
}
