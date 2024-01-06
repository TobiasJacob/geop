use geop_geometry::{
    points::point::Point,
    surface_surface_intersection::surface_surface::{
        surface_surface_intersection, FaceSurfaceIntersection,
    },
};

use crate::topology::{
    edge::Edge,
    face::Face,
    remesh::face::{face_remesh, face_split, FaceSplit}, contour::Contour,
};

pub fn face_face_same_surface_intersection(face_self: &Face, face_other: &Face) -> Face {
    assert!(
        face_self.surface == face_other.surface,
        "Faces must have the same surface",
    );

    let edges = face_split(face_self, face_other)
        .drain(..)
        .filter(|mode| match mode {
            FaceSplit::AinB(_) => true,
            FaceSplit::AonBSameSide(_) => true,
            FaceSplit::AonBOpSide(_) => false,
            FaceSplit::AoutB(_) => false,
            FaceSplit::BinA(_) => true,
            FaceSplit::BonASameSide(_) => false,
            FaceSplit::BonAOpSide(_) => false,
            FaceSplit::BoutA(_) => false,
        })
        .collect::<Vec<FaceSplit>>();

    return face_remesh(face_self.surface.clone(), edges);
}

pub enum FaceFaceIntersection {
    None,
    EdgesAndPoints(Vec<Point>, Vec<Edge>, Vec<Contour>),
    Face(Face),
}

pub fn face_face_intersection(face_self: &Face, face_other: &Face) -> Vec<FaceFaceIntersection> {
    assert!(
        face_self.surface != face_other.surface,
        "Faces must have different surfaces",
    );

    match surface_surface_intersection(&face_self.surface, &face_other.surface) {
        FaceSurfaceIntersection::None => vec![],
        FaceSurfaceIntersection::CurvesAndPoints(_curves, _points) => {
            todo!("face")
            // let mut intersections = vec![];
            // for point in points.iter() {
            //     if face_point_contains(face_self, *point) != FacePointContains::Outside {
            //         if face_point_contains(face_other, *point) != FacePointContains::Outside {
            //             intersections.push(FaceFaceIntersection::Point(Rc::new(point.clone())));
            //         }
            //     }
            // }

            // let mut segments: Vec<EdgeEdgeIntersection> = vec![];
            // for curve in curves.iter() {
            //     let edge = Edge::new(todo!("Start"), todo!("End"), Rc::new(curve.clone()));
            //     segments.push(EdgeEdgeIntersection::Edge(edge));
            // }

            // for face in &[face_self, face_other] {
            //     let mut new_segments = vec![];

            //     for seg in segments.iter() {
            //         match seg {
            //             EdgeEdgeIntersection::Point(p) => {
            //                 if face_point_contains(face, **p) != FacePointContains::Outside {
            //                     new_segments.push(seg.clone());
            //                 }
            //             }
            //             EdgeEdgeIntersection::Edge(e) => {
            //                 new_segments.extend(face_edge_intersection(face_self, e));
            //             }
            //         }
            //     }

            //     segments = new_segments;
            // }

            // segments
            //     .iter()
            //     .map(|seg| match seg {
            //         EdgeEdgeIntersection::Point(p) => FaceFaceIntersection::Point(p.clone()),
            //         EdgeEdgeIntersection::Edge(e) => FaceFaceIntersection::Edge(e.clone()),
            //     })
            //     .collect()
        }
        FaceSurfaceIntersection::Surface(_surface) => {
            vec![FaceFaceIntersection::Face(
                face_face_same_surface_intersection(face_self, face_other),
            )]
        }
    }
}
