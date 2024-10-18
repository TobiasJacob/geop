use geop_topology::{
    contains::volume_point::{volume_point_contains, VolumePointContains},
    topology::{face::Face, volume::Volume},
};

pub enum VolumeFaceContains {
    Inside,
    BoundarySameNormals,
    BoundaryDifferentNormals,
    Outside,
}

pub fn volume_face_contains(volume: &Volume, face: &Face) -> VolumeFaceContains {
    let p = face.inner_point();
    println!("Face {}", face);
    println!("Point {:?}", p);
    match volume_point_contains(volume, p) {
        VolumePointContains::Inside => VolumeFaceContains::Inside,
        VolumePointContains::OnFace(face2) => {
            if face.normal(p).dot(face2.normal(p)) > 0.0 {
                VolumeFaceContains::BoundarySameNormals
            } else {
                VolumeFaceContains::BoundaryDifferentNormals
            }
        }
        VolumePointContains::OnEdge(_) => panic!("Should not happen"),
        VolumePointContains::OnPoint(_) => panic!("Should not happen"),
        VolumePointContains::Outside => VolumeFaceContains::Outside,
    }
}
