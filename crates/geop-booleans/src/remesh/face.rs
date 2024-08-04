use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

use geop_geometry::surfaces::surface::Surface;
use geop_topology::{
    contains::{
        face_contour::{face_contour_contains, FaceContourContains},
        face_edge::{face_edge_contains, FaceEdgeContains},
    },
    topology::{contour::Contour, edge::Edge, face::Face},
};

use crate::{
    intersections::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection},
    split_if_necessary::point_split_edge::split_edges_by_points_if_necessary,
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
                        if let Some(p) = edge.start {
                            intersections.push(p);
                        }
                        if let Some(p) = edge.end {
                            intersections.push(p);
                        }
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

    // println!("intersections: {:}", intersections.len());
    // for point in intersections.iter() {
    //     println!("Point: {:?}", point);
    //     debug_data::add_point(point.clone(), DebugColor::Green);
    // }
    let edges_self = split_edges_by_points_if_necessary(face_self.all_edges(), &intersections);
    let edges_other = split_edges_by_points_if_necessary(face_other.all_edges(), &intersections);

    let res: Vec<FaceSplit> = edges_self
        .into_iter()
        .map(|edge| match face_edge_contains(face_other, &edge) {
            FaceEdgeContains::Inside => FaceSplit::AinB(edge),
            FaceEdgeContains::OnBorderSameDir => FaceSplit::AonBSameSide(edge),
            FaceEdgeContains::OnBorderOppositeDir => FaceSplit::AonBOpSide(edge),
            FaceEdgeContains::Outside => FaceSplit::AoutB(edge),
            FaceEdgeContains::NotSameSurface => panic!("Not same surface"),
        })
        .chain(
            edges_other
                .into_iter()
                .map(|edge| match face_edge_contains(face_self, &edge) {
                    FaceEdgeContains::Inside => FaceSplit::BinA(edge),
                    FaceEdgeContains::OnBorderSameDir => FaceSplit::BonASameSide(edge),
                    FaceEdgeContains::OnBorderOppositeDir => FaceSplit::BonAOpSide(edge),
                    FaceEdgeContains::Outside => FaceSplit::BoutA(edge),
                    FaceEdgeContains::NotSameSurface => panic!("Not same surface"),
                }),
        )
        .collect();

    // for edge in res.iter() {
    //     println!("Edge: {:?}", edge);
    //     match edge {
    //         FaceSplit::AinB(edge) => debug_data::add_edge((edge).clone(), DebugColor::Black),
    //         FaceSplit::AonBSameSide(edge) => debug_data::add_edge((edge).clone(), DebugColor::Red),
    //         FaceSplit::AonBOpSide(edge) => {
    //             debug_data::add_edge((edge).clone(), DebugColor::Transparent)
    //         }
    //         FaceSplit::AoutB(edge) => debug_data::add_edge((edge).clone(), DebugColor::Transparent),
    //         FaceSplit::BinA(edge) => debug_data::add_edge((edge).clone(), DebugColor::Yellow),
    //         FaceSplit::BonASameSide(edge) => {
    //             debug_data::add_edge((edge).clone(), DebugColor::Transparent)
    //         }
    //         FaceSplit::BonAOpSide(edge) => {
    //             debug_data::add_edge((edge).clone(), DebugColor::Transparent)
    //         }
    //         FaceSplit::BoutA(edge) => debug_data::add_edge((edge).clone(), DebugColor::Transparent),
    //     }
    // }

    res
}

pub fn face_remesh(mut edges_intermediate: Vec<FaceSplit>) -> Vec<Contour> {
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

    return contours;
}

pub struct ContourHierarchy {
    pub contour: Contour,
    pub children: Vec<ContourHierarchy>,
}

impl ContourHierarchy {
    // Add contour to this hierarchy if it is inside one of the children or itself
    pub fn consume_contour_if_inside(
        &mut self,
        contour: Contour,
        surface: Rc<Surface>,
    ) -> Option<Contour> {
        for child in self.children.iter_mut() {
            match face_contour_contains(
                &Face::new(Some(child.contour.flip()), vec![], surface.clone()),
                &contour,
            ) {
                FaceContourContains::Inside => {
                    assert!(child
                        .consume_contour_if_inside(contour, surface.clone())
                        .is_none());
                    return None;
                }
                FaceContourContains::Outside => {}
                FaceContourContains::Equals => panic!("should not happen"),
                FaceContourContains::Wiggly => panic!("should not happen"),
                FaceContourContains::NotSameSurface => panic!("should not happen"),
            }
        }
        if face_contour_contains(
            &Face::new(Some(self.contour.clone()), vec![], surface.clone()),
            &contour,
        ) == FaceContourContains::Inside
        {
            self.children.push(ContourHierarchy {
                contour,
                children: Vec::new(),
            });
            return None;
        }
        return Some(contour);
    }

    pub fn as_faces(&self, surface: Rc<Surface>) -> Vec<Face> {
        let mut faces = Vec::<Face>::new();
        let mut face = Face::new(Some(self.contour.clone()), vec![], surface.clone());
        for child in self.children.iter() {
            face.holes.push(child.contour.clone());
            for child2 in child.children.iter() {
                faces.extend(child2.as_faces(surface.clone()));
            }
        }
        faces.push(face);
        faces
    }
}

impl Display for ContourHierarchy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ContourHierarchy({}", self.contour)?;
        for child in self.children.iter() {
            writeln!(f, "  Child: {}", child)?;
        }
        writeln!(f, ")")?;
        Ok(())
    }
}

pub fn normalize_faces(contours: Vec<Contour>, surface: Rc<Surface>) -> Vec<Face> {
    println!("normalize_faces");
    println!("Contours: {}", contours.len());
    let mut hierarchies = Vec::<ContourHierarchy>::new();
    'outer_loop: for contour in contours.iter() {
        for h in hierarchies.iter_mut() {
            if h.consume_contour_if_inside(contour.clone(), surface.clone())
                .is_none()
            {
                continue 'outer_loop;
            }
        }
        hierarchies.push(ContourHierarchy {
            contour: contour.clone(),
            children: Vec::new(),
        });
    }
    // Now build a hierarchy of Contours
    let mut faces = Vec::<Face>::new();
    for h in hierarchies.iter() {
        faces.extend(h.as_faces(surface.clone()));
    }

    println!("Hierarchies: {}", hierarchies.len());
    for h in hierarchies.iter() {
        println!("Hierarchy: {}", h);
    }
    // Now build a hierarchy of Contours
    let mut faces = Vec::<Face>::new();
    for h in hierarchies.iter() {
        faces.extend(h.as_faces(surface.clone()));
    }

    println!("Faces: {}", faces.len());
    for face in faces.iter() {
        println!("Face: {}", face);
    }
    faces
}
