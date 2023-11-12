use std::rc::Rc;

use geop_geometry::{points::point::Point, surfaces::surface};

use crate::topology::{face::{Face, face_surface::FaceSurface}, intersections::edge_edge::EdgeEdgeIntersection, edge::Edge, contains::face_edge::{FaceContainsEdge, face_contains_edge}, contour::Contour};

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

    let mut intersections = Vec::<Rc<Point>>::new();
    for edge in face_self.boundaries.iter() {
        for other_edge in face_other.boundaries.iter() {
            for intersection in edge.intersect_contour(&other_edge) {
                match intersection {
                    EdgeEdgeIntersection::Point(point) => intersections.push(point),
                    EdgeEdgeIntersection::Edge(edge) => {
                        intersections.push(edge.start.clone());
                        intersections.push(edge.end.clone());
                    }
                }
            }
        }
    }

    let mut contours_face = face_self.boundaries.clone();
    let mut contours_other = face_other.boundaries.clone();

    for vert in intersections {
        contours_face = contours_face
            .into_iter()
            .map(|contour| contour.split_if_necessary(*vert))
            .collect();
        contours_other = contours_other
            .into_iter()
            .map(|contour| contour.split_if_necessary(*vert))
            .collect();
    }

    contours_face
        .into_iter()
        .map(|contour| {
            return contour
                .edges
                .into_iter()
                .map(|edge| match face_contains_edge(face_other, &edge) {
                    FaceContainsEdge::Inside => FaceSplit::AinB(edge),
                    FaceContainsEdge::OnBorderSameDir => FaceSplit::AonBSameSide(edge),
                    FaceContainsEdge::OnBorderOppositeDir => FaceSplit::AonBOpSide(edge),
                    FaceContainsEdge::Outside => FaceSplit::AoutB(edge),
                })
                .collect::<Vec<FaceSplit>>();
        })
        .chain(contours_other.into_iter().map(|contour| {
            contour
                .edges
                .into_iter()
                .map(|edge| match face_contains_edge(face_self, &edge) {
                    FaceContainsEdge::Inside => FaceSplit::BinA(edge),
                    FaceContainsEdge::OnBorderSameDir => FaceSplit::BonASameSide(edge),
                    FaceContainsEdge::OnBorderOppositeDir => FaceSplit::BonAOpSide(edge),
                    FaceContainsEdge::Outside => FaceSplit::BoutA(edge),
                })
                .collect::<Vec<FaceSplit>>()
        })).flatten().collect()
}

pub fn face_remesh(surface: Rc<FaceSurface>, mut edges_intermediate: Vec<FaceSplit>) -> Face {
    let mut edges = edges_intermediate.drain(..)
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

    for edge in edges.iter() {
        println!("Edge: {:?}", edge);
    }

    // Now find all the contours
    let mut contours = Vec::<Contour>::new();
    while let Some(current_edge) = edges.pop() {
        let mut new_contour = vec![current_edge];
        loop {
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
