use std::rc::Rc;

use geop_geometry::surfaces::surface::Surface;

use crate::{
    debug_data::{self, DebugColor},
    topology::{
        contains::face_edge::{face_edge_contains, FaceEdgeContains},
        contour::Contour,
        edge::Edge,
        face::Face,
        intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
        split_if_necessary::point_split_edge::split_edges_by_points_if_necessary,
    },
};

use geop_geometry::points::point::Point;

pub fn face_split_points(face_self: &Face, face_other: &Face) -> Vec<Point> {
    let mut intersections = Vec::<Point>::new();
    for es in face_self.all_edges().iter() {
        for eo in face_other.all_edges().iter() {
            match edge_edge_intersection(&es, &eo) {
                EdgeEdgeIntersection::Points(points) => {
                    intersections.extend(points);
                }
                EdgeEdgeIntersection::Edges(edges) => {
                    for edge in edges {
                        intersections.push(edge.start.clone());
                        intersections.push(edge.end.clone());
                    }
                }
                EdgeEdgeIntersection::None => {}
            }
        }
    }

    intersections
}

#[derive(Debug)]
pub enum FaceSplit {
    AinB(Edge),
    AonBSameSide(Edge),
    AonBOpSide(Edge),
    AoutB(Edge),
    BinA(Edge),
    BonASameSide(Edge),
    BonAOpSide(Edge),
    BoutA(Edge),
}

pub fn face_split(face_self: &Face, face_other: &Face) -> Vec<FaceSplit> {
    assert!(face_self.surface == face_other.surface);
    println!("Face_self {:}", face_self);
    println!("Face_other {:}", face_other);

    // debug_data::add_face(face_self.clone(), DebugColor::Red);
    // debug_data::add_face(face_other.clone(), DebugColor::Blue);

    let intersections = face_split_points(face_self, face_other);

    println!("intersections: {:}", intersections.len());
    for point in intersections.iter() {
        println!("Point: {:?}", point);
        debug_data::add_point(point.clone(), DebugColor::Green);
    }
    let edges_self = split_edges_by_points_if_necessary(face_self.all_edges(), &intersections);
    let edges_other = split_edges_by_points_if_necessary(face_other.all_edges(), &intersections);

    let res: Vec<FaceSplit> = edges_self
        .into_iter()
        .map(|edge| match face_edge_contains(face_other, &edge) {
            FaceEdgeContains::Inside => FaceSplit::AinB(edge),
            FaceEdgeContains::OnBorderSameDir => FaceSplit::AonBSameSide(edge),
            FaceEdgeContains::OnBorderOppositeDir => FaceSplit::AonBOpSide(edge),
            FaceEdgeContains::Outside => FaceSplit::AoutB(edge),
        })
        .chain(
            edges_other
                .into_iter()
                .map(|edge| match face_edge_contains(face_self, &edge) {
                    FaceEdgeContains::Inside => FaceSplit::BinA(edge),
                    FaceEdgeContains::OnBorderSameDir => FaceSplit::BonASameSide(edge),
                    FaceEdgeContains::OnBorderOppositeDir => FaceSplit::BonAOpSide(edge),
                    FaceEdgeContains::Outside => FaceSplit::BoutA(edge),
                }),
        )
        .collect();

    for edge in res.iter() {
        println!("Edge: {:?}", edge);
        match edge {
            FaceSplit::AinB(edge) => debug_data::add_edge((edge).clone(), DebugColor::Black),
            FaceSplit::AonBSameSide(edge) => debug_data::add_edge((edge).clone(), DebugColor::Red),
            FaceSplit::AonBOpSide(edge) => {
                debug_data::add_edge((edge).clone(), DebugColor::Transparent)
            }
            FaceSplit::AoutB(edge) => debug_data::add_edge((edge).clone(), DebugColor::Transparent),
            FaceSplit::BinA(edge) => debug_data::add_edge((edge).clone(), DebugColor::Yellow),
            FaceSplit::BonASameSide(edge) => {
                debug_data::add_edge((edge).clone(), DebugColor::Transparent)
            }
            FaceSplit::BonAOpSide(edge) => {
                debug_data::add_edge((edge).clone(), DebugColor::Transparent)
            }
            FaceSplit::BoutA(edge) => debug_data::add_edge((edge).clone(), DebugColor::Transparent),
        }
    }

    res
}

pub fn face_remesh(surface: Rc<Surface>, mut edges_intermediate: Vec<FaceSplit>) -> Face {
    println!("face_remesh");
    for edge in edges_intermediate.iter() {
        println!("Edge: {:?}", edge);
    }
    let mut edges = edges_intermediate
        .drain(..)
        .map(|e| match e {
            FaceSplit::AinB(edge) => edge,
            FaceSplit::AonBSameSide(edge) => edge,
            FaceSplit::AonBOpSide(edge) => edge,
            FaceSplit::AoutB(edge) => edge,
            FaceSplit::BinA(edge) => edge,
            FaceSplit::BonASameSide(edge) => edge,
            FaceSplit::BonAOpSide(edge) => edge,
            FaceSplit::BoutA(edge) => edge,
        })
        .collect::<Vec<Edge>>();
    // Now find all the contours
    let mut contours = Vec::<Contour>::new();
    while let Some(current_edge) = edges.pop() {
        let mut new_contour = vec![current_edge];
        loop {
            println!("new_contour");
            for edge in new_contour.iter() {
                println!("Edge: {:?}", edge);
            }
            println!("edges");
            for edge in edges.iter() {
                println!("Edge: {:?}", edge);
            }
            let next_i = edges.iter().position(|edge| {
                edge.start == new_contour[new_contour.len() - 1].end
                    || edge.end == new_contour[new_contour.len() - 1].end
            });
            match next_i {
                Some(i) => {
                    if edges[i].start == new_contour[new_contour.len() - 1].end {
                        new_contour.push(edges.remove(i));
                    } else {
                        new_contour.push(edges.remove(i).flip());
                    }
                }
                None => {
                    assert!(new_contour[0].start == new_contour[new_contour.len() - 1].end);
                    contours.push(Contour::new(new_contour));
                    break;
                }
            }
        }
    }

    return Face::new(contours, surface.clone());
}
