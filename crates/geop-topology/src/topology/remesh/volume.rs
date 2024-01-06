use std::rc::Rc;

use crate::topology::{
    face::Face, volume::Volume, contour::Contour, intersections::face_face::{face_face_intersection, FaceFaceIntersection}, edge::Edge, split_if_necessary::edge_split_face::split_faces_by_edges_if_necessary, contains::volume_face::{volume_face_contains, VolumeFaceContains},
};

pub fn volume_split_edges(volume_self: &Volume, volume_other: &Volume) -> Vec<Edge> {
    let mut edges = Vec::<Edge>::new();
    for face_self in volume_self.faces.iter() {
        for face_other in volume_other.faces.iter() {
            let intersection = face_face_intersection(face_self, face_other);
            match intersection {
                FaceFaceIntersection::None => {}
                FaceFaceIntersection::EdgesAndPoints(_points, new_edges, contours) => {
                    for cont in contours {
                        edges.extend(cont.edges);
                    }
                    edges.extend(new_edges);
                }
                FaceFaceIntersection::Face(face) => {
                    for cont in face.boundaries {
                        edges.extend(cont.edges);
                    }
                }
            }
        }
    }
    todo!("Remesh all edges to contours and return contours")
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
    let intersections = volume_split_edges(volume_self, volume_other);

    let faces_self = volume_self.faces.clone();
    let faces_other = volume_other.faces.clone();

    let faces_self = split_faces_by_edges_if_necessary(faces_self, &intersections);
    let faces_other = split_faces_by_edges_if_necessary(faces_other, &intersections);

    let mut result = Vec::<VolumeSplit>::new();
    for face_self in faces_self {
        match volume_face_contains(volume_other, &face_self) {
            VolumeFaceContains::Inside => {
                result.push(VolumeSplit::AinB(face_self));
            }
            VolumeFaceContains::Outside => {
                result.push(VolumeSplit::AoutB(face_self));
            }
            VolumeFaceContains::OnBorderSameDir => {
                result.push(VolumeSplit::AonBSameSide(face_self));
            }
            VolumeFaceContains::OnBorderOppositeDir => {
                result.push(VolumeSplit::AonBOpSide(face_self));
            }
        }
    }

    for face_other in faces_other {
        match volume_face_contains(volume_self, &face_other) {
            VolumeFaceContains::Inside => {
                result.push(VolumeSplit::BinA(face_other));
            }
            VolumeFaceContains::Outside => {
                result.push(VolumeSplit::BoutA(face_other));
            }
            VolumeFaceContains::OnBorderSameDir => {
                result.push(VolumeSplit::BonASameSide(face_other));
            }
            VolumeFaceContains::OnBorderOppositeDir => {
                result.push(VolumeSplit::BonAOpSide(face_other));
            }
        }
    }

    result
}

pub fn volume_remesh(mut faces_intermediate: Vec<Face>) -> Volume {
    let mut faces = Vec::<Face>::new();
    for face in faces_intermediate.drain(..) {
        faces.push(face);
    }
    Volume::new(faces)
}
