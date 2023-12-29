use std::rc::Rc;

use crate::topology::{
    face::Face, intersections::shell_shell::shell_shell_intersect, volume::Volume,
};

#[derive(Debug)]
pub enum VolumeSplit {
    AinB(Rc<Face>),
    AonBSameSide(Rc<Face>),
    AonBOpSide(Rc<Face>),
    AoutB(Rc<Face>),
    BinA(Rc<Face>),
    BonASameSide(Rc<Face>),
    BonAOpSide(Rc<Face>),
    BoutA(Rc<Face>),
}

pub fn volume_split(volume_self: &Volume, volume_other: &Volume) -> Vec<VolumeSplit> {
    let mut _intersections = shell_shell_intersect(volume_self, volume_other);

    let mut _faces_self = volume_self.faces.clone();
    let mut _faces_other = volume_other.faces.clone();
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
    // }

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
