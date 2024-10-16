// use crate::{
//     remesh::face::normalize_faces,
//     split_if_necessary::point_split_edge::split_contours_by_points_if_necessary,
// };

use geop_geometry::points::point::Point;
use geop_topology::{
    contains::{
        contour_point::contour_point_contains,
        edge_point::EdgePointContains,
        face_edge::{face_edge_contains, FaceEdgeContains},
    },
    topology::{contour::Contour, edge::Edge, face::Face},
};

use crate::remesh::face::normalize_faces;

use super::point_split_edge::split_contours_by_points_if_necessary;

pub fn split_face_by_edge_if_necessary(face: &Face, edge: &Edge) -> Vec<Face> {
    match face_edge_contains(face, edge) {
        FaceEdgeContains::Inside => {
            if edge.start.is_none() || edge.end.is_none() {
                todo!("Not yet implemented. Edge should have start and end points");
            }

            let split_points = vec![edge.start.unwrap(), edge.end.unwrap()];

            let mut contours = face.holes.clone();
            if let Some(boundary) = face.boundary.clone() {
                contours.push(boundary);
            }
            let contours = split_contours_by_points_if_necessary(contours, &split_points);
            let start_contour =
                contours
                    .iter()
                    .find(|&c| match contour_point_contains(c, edge.start.unwrap()) {
                        EdgePointContains::OnPoint(_) => true,
                        _ => false,
                    });
            let end_contour =
                contours
                    .iter()
                    .find(|&c| match contour_point_contains(c, edge.end.unwrap()) {
                        EdgePointContains::OnPoint(_) => true,
                        _ => false,
                    });
            let mut new_contours = Vec::<Contour>::new();
            match start_contour {
                Option::Some(start_contour) => match end_contour {
                    Option::Some(end_contour) => {
                        if std::ptr::eq(start_contour, end_contour) {
                            println!("Start contour");
                            for e in start_contour.edges.iter() {
                                println!("{}", e);
                            }
                            // Make it 2 contours
                            let mut edges =
                                start_contour.get_subcurve(edge.end.unwrap(), edge.start.unwrap());
                            println!("Edges");
                            for e in edges.iter() {
                                println!("{:}", e);
                            }
                            println!("Edge");
                            edges.push(edge.clone());
                            println!("{:}", edge);
                            new_contours.push(Contour::new(edges));
                            let mut edges =
                                start_contour.get_subcurve(edge.start.unwrap(), edge.end.unwrap());
                            edges.push(edge.flip());
                            new_contours.push(Contour::new(edges));
                            // Push the rest of the contours
                            for contour in contours.iter() {
                                if !std::ptr::eq(contour, start_contour) {
                                    new_contours.push(contour.clone());
                                }
                            }
                        } else {
                            // Make it 1 contour
                            let mut edges =
                                start_contour.get_subcurve_single_point(edge.start.unwrap());
                            edges.push(edge.clone());
                            edges.extend(end_contour.get_subcurve_single_point(edge.end.unwrap()));
                            edges.push(edge.flip());
                            new_contours.push(Contour::new(edges));
                        }
                    }
                    Option::None => {
                        // Make it 1 contour
                        let mut edges =
                            start_contour.get_subcurve_single_point(edge.start.unwrap());
                        edges.push(edge.clone());
                        edges.push(edge.flip());
                        new_contours.push(Contour::new(edges));
                    }
                },
                Option::None => match end_contour {
                    Option::Some(end_contour) => {
                        // Make it 1 contour
                        let mut edges = end_contour.get_subcurve_single_point(edge.end.unwrap());
                        edges.push(edge.clone());
                        edges.push(edge.flip());
                        new_contours.push(Contour::new(edges));
                    }
                    Option::None => {
                        // Make it 1 contour
                        let edges = vec![edge.clone(), edge.flip()];
                        new_contours.push(Contour::new(edges));
                    }
                },
            }
            return normalize_faces(new_contours, face.surface.clone());
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
