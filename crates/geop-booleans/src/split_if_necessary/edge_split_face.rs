use crate::{
    remesh::face::normalize_faces,
    split_if_necessary::point_split_edge::split_contours_by_points_if_necessary,
};

use geop_topology::{
    contains::{
        contour_point::contour_point_contains,
        edge_point::EdgePointContains,
        face_edge::{face_edge_contains, FaceEdgeContains},
    },
    topology::{contour::Contour, edge::Edge, face::Face},
};

pub fn split_face_by_edge_if_necessary(face: &Face, edge: &Edge) -> Vec<Face> {
    match face_edge_contains(face, edge) {
        FaceEdgeContains::Inside => {
            todo!()
            // let split_points = vec![edge.start.clone(), edge.end.clone()]
            //     .drain(..)
            //     .filter_map(|p| p)
            //     .collect();

            // let mut contours = face.holes.clone();
            // contours.push(face.boundary.clone());
            // let contours = split_contours_by_points_if_necessary(contours, &split_points);
            // let start_contour =
            //     contours
            //         .iter()
            //         .find(|&c| match contour_point_contains(c, edge.start) {
            //             EdgePointContains::OnPoint(_) => true,
            //             _ => false,
            //         });
            // let end_contour =
            //     contours
            //         .iter()
            //         .find(|&c| match contour_point_contains(c, edge.end) {
            //             EdgePointContains::OnPoint(_) => true,
            //             _ => false,
            //         });
            // let mut new_contours = Vec::<Contour>::new();
            // match start_contour {
            //     Option::Some(start_contour) => match end_contour {
            //         Option::Some(end_contour) => {
            //             if std::ptr::eq(start_contour, end_contour) {
            //                 // Make it 2 contours
            //                 let mut edges = start_contour.get_subcurve(edge.end, edge.start);
            //                 edges.push(edge.clone());
            //                 new_contours.push(Contour::new(edges));
            //                 let mut edges = start_contour.get_subcurve(edge.start, edge.end);
            //                 edges.push(edge.flip());
            //                 new_contours.push(Contour::new(edges));
            //                 // Push the rest of the contours
            //                 for contour in contours.iter() {
            //                     if !std::ptr::eq(contour, start_contour) {
            //                         new_contours.push(contour.clone());
            //                     }
            //                 }
            //             } else {
            //                 // Make it 1 contour
            //                 let mut edges = start_contour.get_subcurve_single_point(edge.start);
            //                 edges.push(edge.clone());
            //                 edges.extend(end_contour.get_subcurve_single_point(edge.end));
            //                 edges.push(edge.flip());
            //                 new_contours.push(Contour::new(edges));
            //             }
            //         }
            //         Option::None => {
            //             // Make it 1 contour
            //             let mut edges = start_contour.get_subcurve_single_point(edge.start);
            //             edges.push(edge.clone());
            //             edges.push(edge.flip());
            //             new_contours.push(Contour::new(edges));
            //         }
            //     },
            //     Option::None => match end_contour {
            //         Option::Some(end_contour) => {
            //             // Make it 1 contour
            //             let mut edges = end_contour.get_subcurve_single_point(edge.end);
            //             edges.push(edge.clone());
            //             edges.push(edge.flip());
            //             new_contours.push(Contour::new(edges));
            //         }
            //         Option::None => {
            //             // Make it 1 contour
            //             let edges = vec![edge.clone(), edge.flip()];
            //             new_contours.push(Contour::new(edges));
            //         }
            //     },
            // }
            // return normalize_faces(new_contours, face.surface.clone());
        }
        FaceEdgeContains::Outside => {
            vec![face.clone()]
        }
        FaceEdgeContains::OnBorderSameDir => {
            vec![face.clone()]
        }
        FaceEdgeContains::OnBorderOppositeDir => {
            vec![face.clone()]
        }
        FaceEdgeContains::NotSameSurface => {
            vec![face.clone()]
        }
    }
}

pub fn split_face_by_edges_if_necessary(face: &Face, edges: &[Edge]) -> Vec<Face> {
    let mut result = vec![face.clone()];
    for c in edges {
        let mut new_result = Vec::<Face>::new();
        for face in result.iter() {
            new_result.extend(split_face_by_edge_if_necessary(face, c));
        }
        result = new_result;
    }
    result
}

pub fn split_faces_by_edges_if_necessary(faces: Vec<Face>, edges: &Vec<Edge>) -> Vec<Face> {
    let mut result = Vec::<Face>::new();
    for face in faces {
        result.extend(split_face_by_edges_if_necessary(&face, &edges));
    }
    result
}
