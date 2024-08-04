use crate::intersections::face_face::{face_face_intersection, FaceFaceIntersection};
use geop_topology::topology::{edge::Edge, face::Face, volume::Volume};

// Points are ignored for now.
pub fn volume_split_edges(volume_self: &Volume, volume_other: &Volume) -> Vec<Edge> {
    let mut edges = Vec::<Edge>::new();
    for face_self in volume_self.all_faces().iter() {
        for face_other in volume_other.all_faces().iter() {
            match face_face_intersection(face_self, face_other) {
                FaceFaceIntersection::EdgesAndPoints(_points, new_edges) => {
                    edges.extend(new_edges);
                }
                FaceFaceIntersection::Faces(faces) => {
                    for edge in faces.into_iter().flat_map(|face| face.all_edges()) {
                        edges.push(edge);
                    }
                }
                FaceFaceIntersection::None => {}
            }
        }
    }
    edges
}

#[derive(Debug)]
pub enum VolumeSplit {
    AinB(Face),
    AonBSameSide(Face),
    AonBOpSide(Face),
    AoutB(Face),
    BinA(Face),
    BonASameSide(Face),
    BonAOpSide(Face),
    BoutA(Face),
}

pub fn volume_split(_volume_self: &Volume, _volume_other: &Volume) -> Vec<VolumeSplit> {
    todo!("Implement volume split");
    // let mut intersections = volume_split_edges(volume_self, volume_other);

    // let mut faces_self = split_faces_by_edges_if_necessary(volume_self.all_faces(), &intersections);
    // let mut faces_other =
    //     split_faces_by_edges_if_necessary(volume_other.all_faces(), &intersections);

    // faces_self
    //     .into_iter()
    //     .map(|edge| match volume_contains_face(face_other, &edge) {
    //         FaceContainsEdge::Inside => VolumeSplit::AinB(edge),
    //         FaceContainsEdge::OnBorderSameDir => VolumeSplit::AonBSameSide(edge),
    //         FaceContainsEdge::OnBorderOppositeDir => VolumeSplit::AonBOpSide(edge),
    //         FaceContainsEdge::Outside => VolumeSplit::AoutB(edge),
    //     })
    //     .chain(
    //         faces_other
    //             .into_iter()
    //             .map(|face| match volume_contains_face(face_self, &edge) {
    //                 FaceContainsEdge::Inside => VolumeSplit::BinA(edge),
    //                 FaceContainsEdge::OnBorderSameDir => VolumeSplit::BonASameSide(edge),
    //                 FaceContainsEdge::OnBorderOppositeDir => VolumeSplit::BonAOpSide(edge),
    //                 FaceContainsEdge::Outside => VolumeSplit::BoutA(edge),
    //             }),
    //     )
    //     .collect()
}
