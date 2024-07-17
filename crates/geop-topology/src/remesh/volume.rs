use crate::{
    intersections::face_face::{face_face_intersection, FaceFaceIntersection},
    split_if_necessary::edge_split_face::split_faces_by_contours_if_necessary,
    topology::{contour::Contour, edge::Edge, face::Face, volume::Volume},
};

// Points are ignored for now. In theory we could split by contours, but its simpler to just split by edges.
pub fn volume_split_contours(volume_self: &Volume, volume_other: &Volume) -> Vec<Contour> {
    let mut edges = Vec::<Edge>::new();
    for face_self in volume_self.all_faces().iter() {
        for face_other in volume_other.all_faces().iter() {
            match face_face_intersection(face_self, face_other) {
                FaceFaceIntersection::EdgesAndPoints(points, new_edges) => {
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
    let mut result = Vec::<Contour>::new();
    loop {
        let mut contour = Vec::<Edge>::new();
        contour.push(edges.pop().unwrap());
        loop {
            let last_edge = contour.last().unwrap();
            if last_edge.end == contour.first().unwrap().start {
                break;
            }
            // Find the next edge that starts where the last edge ends
            let next_edge = edges
                .iter()
                .position(|edge| edge.start == last_edge.end || edge.end == last_edge.end);
            match next_edge {
                Some(index) => {
                    let edge = edges.swap_remove(index);
                    if edge.start == last_edge.end {
                        contour.push(edge);
                    } else {
                        contour.push(edge.flip());
                    }
                }
                None => {
                    break;
                }
            }
        }
        if contour.last().unwrap().end == contour.first().unwrap().start {
            result.push(Contour::new(contour));
        }
        if edges.is_empty() {
            break;
        }
    }
    result
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
    let mut intersections = volume_split_contours(volume_self, volume_other);

    let mut faces_self =
        split_faces_by_contours_if_necessary(volume_self.all_faces(), &intersections);
    let mut faces_other =
        split_faces_by_contours_if_necessary(volume_other.all_faces(), &intersections);

    todo!("Implement volume split");

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
