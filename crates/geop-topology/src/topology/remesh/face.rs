use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::{topology::{
    contains::face_edge::{face_contains_edge, FaceContainsEdge},
    contour::Contour,
    edge::Edge,
    face::{face_surface::FaceSurface, Face},
    intersections::{edge_edge::EdgeEdgeIntersection, contour_contour::countour_contour_intersection_points}, split_if_necessary::point_split_edge::split_edges_by_point_if_necessary,
}, debug_data::{self, DebugColor}};

#[derive(Debug)]
pub enum FaceSplit {
    AinB(Rc<Edge>),
    AonBSameSide(Rc<Edge>),
    AonBOpSide(Rc<Edge>),
    AoutB(Rc<Edge>),
    BinA(Rc<Edge>),
    BonASameSide(Rc<Edge>),
    BonAOpSide(Rc<Edge>),
    BoutA(Rc<Edge>),
}

pub fn face_split(face_self: &Face, face_other: &Face) -> Vec<FaceSplit> {
    assert!(face_self.surface == face_other.surface);
    println!("Face_self {:}", face_self);
    println!("Face_other {:}", face_other);

    // debug_data::add_face(face_self.clone(), DebugColor::Red);
    // debug_data::add_face(face_other.clone(), DebugColor::Blue);

    let intersections = countour_contour_intersection_points(face_self, face_other);

    let mut edges_self = face_self.all_edges();
    let mut edges_other = face_other.all_edges();

    println!("intersections: {:}", intersections.len());
    for point in intersections {
        println!("point: {:?}", point);
        edges_self = split_edges_by_point_if_necessary(edges_self, point.clone());
        edges_other = split_edges_by_point_if_necessary(edges_other, point.clone());
    }

    let res: Vec<FaceSplit>  = edges_self
        .into_iter()
        .map(|edge| match face_contains_edge(face_other, &edge) {
            FaceContainsEdge::Inside => FaceSplit::AinB(edge),
            FaceContainsEdge::OnBorderSameDir => FaceSplit::AonBSameSide(edge),
            FaceContainsEdge::OnBorderOppositeDir => FaceSplit::AonBOpSide(edge),
            FaceContainsEdge::Outside => FaceSplit::AoutB(edge),
        })
        .chain(
            edges_other
                .into_iter()
                .map(|edge| match face_contains_edge(face_self, &edge) {
                    FaceContainsEdge::Inside => FaceSplit::BinA(edge),
                    FaceContainsEdge::OnBorderSameDir => FaceSplit::BonASameSide(edge),
                    FaceContainsEdge::OnBorderOppositeDir => FaceSplit::BonAOpSide(edge),
                    FaceContainsEdge::Outside => FaceSplit::BoutA(edge),
                }),
        )
        .collect();

    for edge in res.iter() {
        println!("Edge: {:?}", edge);
        match edge {
            FaceSplit::AinB(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Black),
            FaceSplit::AonBSameSide(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Red),
            FaceSplit::AonBOpSide(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Transparent),
            FaceSplit::AoutB(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Transparent),
            FaceSplit::BinA(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Yellow),
            FaceSplit::BonASameSide(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Transparent),
            FaceSplit::BonAOpSide(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Transparent),
            FaceSplit::BoutA(edge) => debug_data::add_edge((**edge).clone(), DebugColor::Transparent),
        }
    }

    res
}

pub fn face_remesh(surface: Rc<FaceSurface>, mut edges_intermediate: Vec<FaceSplit>) -> Face {
    println!("new_contour");
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
        .collect::<Vec<Rc<Edge>>>();
    // Now find all the contours
    let mut contours = Vec::<Contour>::new();
    while let Some(current_edge) = edges.pop() {
        let mut new_contour = vec![current_edge];
        loop {
            // println!("new_contour");
            // for edge in new_contour.iter() {
            //     println!("Edge: {:?}", edge);
            // }
            // println!("edges");
            // for edge in edges.iter() {
            //     println!("Edge: {:?}", edge);
            // }
            let next_i = edges.iter().position(|edge| {
                edge.start == new_contour[new_contour.len() - 1].end
                    || edge.end == new_contour[new_contour.len() - 1].end
            });
            match next_i {
                Some(i) => {
                    if edges[i].start == new_contour[new_contour.len() - 1].end {
                        new_contour.push(edges.remove(i));
                    } else {
                        new_contour.push(Rc::new(edges.remove(i).neg()));
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
