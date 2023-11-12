use std::rc::Rc;

use geop_geometry::{points::point::Point, surfaces::surface};

use crate::topology::{face::{Face, face_surface::FaceSurface}, intersections::edge_edge::EdgeEdgeIntersection, edge::Edge, contains::face_edge::{FaceContainsEdge, face_contains_edge}, contour::Contour};

#[derive(Debug)]
pub enum EdgeSplit {
    AinB(Rc<Edge>),
    AonBSameSide(Rc<Edge>),
    AonBOpSide(Rc<Edge>),
    AoutB(Rc<Edge>),
    BinA(Rc<Edge>),
    BonASameSide(Rc<Edge>),
    BonAOpSide(Rc<Edge>),
    BoutA(Rc<Edge>),
}


pub fn face_split(face_self: &Face, face_other: &Face) -> Vec<EdgeSplit> {
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
                    FaceContainsEdge::Inside => EdgeSplit::AinB(edge),
                    FaceContainsEdge::OnBorderSameDir => EdgeSplit::AonBSameSide(edge),
                    FaceContainsEdge::OnBorderOppositeDir => EdgeSplit::AonBOpSide(edge),
                    FaceContainsEdge::Outside => EdgeSplit::AoutB(edge),
                })
                .collect::<Vec<EdgeSplit>>();
        })
        .chain(contours_other.into_iter().map(|contour| {
            contour
                .edges
                .into_iter()
                .map(|edge| match face_contains_edge(face_self, &edge) {
                    FaceContainsEdge::Inside => EdgeSplit::BinA(edge),
                    FaceContainsEdge::OnBorderSameDir => EdgeSplit::BonASameSide(edge),
                    FaceContainsEdge::OnBorderOppositeDir => EdgeSplit::BonAOpSide(edge),
                    FaceContainsEdge::Outside => EdgeSplit::BoutA(edge),
                })
                .collect::<Vec<EdgeSplit>>()
        })).flatten().collect()
}

pub fn face_remesh(surface: Rc<FaceSurface>, mut edges_intermediate: Vec<EdgeSplit>) -> Face {
    let mut edges = edges_intermediate.drain(..)
        .map(|e| match e {
            EdgeSplit::AinB(edge) => edge,
            EdgeSplit::AonBSameSide(edge) => edge,
            EdgeSplit::AonBOpSide(edge) => edge,
            EdgeSplit::AoutB(edge) => edge,
            EdgeSplit::BinA(edge) => edge,
            EdgeSplit::BonASameSide(edge) => edge,
            EdgeSplit::BonAOpSide(edge) => edge,
            EdgeSplit::BoutA(edge) => edge,
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
