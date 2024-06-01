use std::rc::Rc;

use geop_geometry::points;

use crate::{
    intersections::face_face::{face_face_intersection, FaceFaceIntersection},
    remesh::edge,
    topology::{contour::Contour, face::Face, volume::Volume},
};

pub fn volume_split_contours(volume_self: &Volume, volume_other: &Volume) -> Vec<Contour> {
    for face_self in volume_self.faces.iter() {
        for face_other in volume_other.faces.iter() {
            match face_face_intersection(face_self, face_other) {
                FaceFaceIntersection::EdgesAndPoints(points, edge) => {}
                FaceFaceIntersection::Faces(faces) => {}
                FaceFaceIntersection::None => {}
            }
        }
    }

    todo!()
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

pub fn volume_split(volume_self: &Volume, volume_other: &Volume) -> Vec<VolumeSplit> {
    // let mut _intersections = shell_shell_intersection(volume_self, volume_other);

    // let mut _faces_self = volume_self.faces.clone();
    // let mut _faces_other = volume_other.faces.clone();
    todo!("Volume::split")
    // for vert in intersections {
    //     faces_self = faces_self
    //         .into_iter()
    //         .map(|face| face.split_if_necessary(*vert))
    //         .collect();
    //     faces_other = faces_other
    //         .into_iter()
    //         .map(|face| face.split_if_necessary(*vert))
    //         .collect();
    // }is_from_inside

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
