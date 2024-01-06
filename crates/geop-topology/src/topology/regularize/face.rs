use std::ops::Index;

use crate::topology::{contains::contour_point::contour_point_contains, face::Face};

type RegularFace = Face; // Faces with exactly one outer boundary and n inner boundaries. The outer boundary comes first.

pub struct ContainmentMatrix {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
}

impl ContainmentMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![false; rows * cols];
        Self { rows, cols, data }
    }
}

impl Index<(usize, usize)> for ContainmentMatrix {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

pub fn face_regularize(face: &Face) -> Vec<RegularFace> {
    let mut contains_matrix = ContainmentMatrix::new(face.boundaries.len(), face.boundaries.len());
    for (i, contour_a) in face.boundaries.iter().enumerate() {
        for (j, contour_b) in face.boundaries.iter().enumerate() {
            if i == j {
                continue;
            }
            contains_matrix[(i, j)] =
                contour_point_contains(contour_a, &contour_b.edges[0].boundaries[0].0).is_inside();
        }
    }

    let mut regular_faces = Vec::<RegularFace>::new();

    todo!("Now decompose the face into regular faces.")
}

pub fn face_regularize_all(faces: Vec<Face>) -> Vec<RegularFace> {
    let mut regular_faces = Vec::<RegularFace>::new();
    for face in faces.iter() {
        regular_faces.extend(face_regularize(face));
    }
    regular_faces
}

pub fn face_is_regular(face: &Face) -> bool {
    todo!("Check if face is regular.")
}
