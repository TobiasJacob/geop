use crate::{
    contains::volume_face::{volume_face_contains, VolumeFaceContains},
    intersections::face_face::{face_face_intersection, FaceFaceIntersection},
    split_if_necessary::edge_split_face::split_faces_by_edges_if_necessary,
};
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

impl VolumeSplit {
    pub fn face(&self) -> &Face {
        match self {
            VolumeSplit::AinB(face) => face,
            VolumeSplit::AonBSameSide(face) => face,
            VolumeSplit::AonBOpSide(face) => face,
            VolumeSplit::AoutB(face) => face,
            VolumeSplit::BinA(face) => face,
            VolumeSplit::BonASameSide(face) => face,
            VolumeSplit::BonAOpSide(face) => face,
            VolumeSplit::BoutA(face) => face,
        }
    }
}

pub fn volume_split(volume_self: &Volume, volume_other: &Volume) -> Vec<VolumeSplit> {
    let intersections = volume_split_edges(volume_self, volume_other);

    let faces_self = split_faces_by_edges_if_necessary(volume_self.all_faces(), &intersections);
    let faces_other = split_faces_by_edges_if_necessary(volume_other.all_faces(), &intersections);

    faces_self
        .into_iter()
        .map(|face| match volume_face_contains(volume_other, &face) {
            VolumeFaceContains::Inside => VolumeSplit::AinB(face),
            VolumeFaceContains::BoundarySameNormals => VolumeSplit::AonBSameSide(face),
            VolumeFaceContains::BoundaryDifferentNormals => VolumeSplit::AonBOpSide(face),
            VolumeFaceContains::Outside => VolumeSplit::AoutB(face),
        })
        .chain(
            faces_other
                .into_iter()
                .map(|face| match volume_face_contains(volume_self, &face) {
                    VolumeFaceContains::Inside => VolumeSplit::BinA(face),
                    VolumeFaceContains::BoundarySameNormals => VolumeSplit::BonASameSide(face),
                    VolumeFaceContains::BoundaryDifferentNormals => VolumeSplit::BonAOpSide(face),
                    VolumeFaceContains::Outside => VolumeSplit::BoutA(face),
                }),
        )
        .collect()
}
