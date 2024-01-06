use std::rc::Rc;

use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
};

use crate::topology::{
    edge::Edge, face::Face, volume::Volume,
};

use super::face_point::{face_point_contains, FacePointContains};

pub enum VolumePointContains {
    Inside,
    OnFace(Rc<Face>),
    OnEdge(Rc<Edge>),
    OnPoint(Rc<Point>),
    Outside,
}

pub fn volume_point_contains(volume: &Volume, other: &Rc<Point>) -> VolumePointContains {
    // first check if point is on any other face
    for face in volume.faces.iter() {
        match face_point_contains(face, other) {
            FacePointContains::Inside => return VolumePointContains::OnFace(face.clone()),
            FacePointContains::OnEdge(edge) => return VolumePointContains::OnEdge(edge),
            FacePointContains::OnPoint(point) => return VolumePointContains::OnPoint(point),
            FacePointContains::Outside => {}
        }
    }

    // choose a random point on a face
    let q = Rc::new(volume.faces[0].inner_point());
    let curve = Edge::new_line(other.clone(), q);

    // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
    for face in volume.faces.iter() {
        let intersections = todo!("Find intersections with face boundaries"); //face.intersect_edge(&curve);
    }
    let mut closest_distance = (**other - *q).norm();
    let curve_dir = *q - **other;
    let normal = volume.normal(*q);
    let mut closest_intersect_from_inside = normal.is_from_inside(curve_dir);
    // for face in volume.faces.iter() {
    //     let edge_intersections: Vec<EdgeEdgeIntersectionOld> =
    //         todo!("Find intersections with face boundaries"); //face.intersect_edge(&curve);
    //     let mut intersections = Vec::<Point>::new();
    //     for intersection in edge_intersections {
    //         match intersection {
    //             EdgeEdgeIntersectionOld::Point(point) => {
    //                 intersections.push(*point);
    //             }
    //             EdgeEdgeIntersectionOld::Edge(edge) => {
    //                 intersections.push(*edge.start);
    //                 intersections.push(*edge.end);
    //             }
    //         }
    //     }
    //     for point in intersections {
    //         let distance = (other - point).norm();
    //         if distance < closest_distance {
    //             let curve_dir = curve.tangent(point);
    //             let normal = volume.normal(point);
    //             closest_distance = distance;
    //             closest_intersect_from_inside = normal.is_from_inside(curve_dir);
    //         }
    //     }
    // }

    match closest_intersect_from_inside {
        true => VolumePointContains::Inside,
        false => VolumePointContains::Outside,
    };

    todo!(  "Fix this")
}
