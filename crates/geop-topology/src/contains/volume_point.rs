use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    curve_surface_intersection::curve_surface::{
        curve_surface_intersection, CurveSurfaceIntersection,
    },
    curves::{bounds::Bounds, curve::Curve, line::Line, CurveLike},
    point::Point,
};

use crate::topology::{
    contour::{contour_no_point::ContourNoPoint, Contour},
    edge::Edge,
    face::Face,
    volume::Volume,
};

use super::face_point::{face_point_contains, FacePointContains};

pub enum VolumePointContains {
    Inside,
    OnFace(Face),
    OnCurveContour(ContourNoPoint),
    OnEdge(Edge),
    OnPoint(Point),
    Outside,
}

pub fn volume_point_contains(volume: &Volume, other: Point) -> VolumePointContains {
    // first check if point is on any other face
    for face in volume.all_faces().iter() {
        match face_point_contains(face, other) {
            FacePointContains::Inside => return VolumePointContains::OnFace(face.clone()),
            FacePointContains::OnCurveContour(curve) => {
                return VolumePointContains::OnCurveContour(curve)
            }
            FacePointContains::OnEdge(edge) => return VolumePointContains::OnEdge(edge),
            FacePointContains::OnPoint(point) => return VolumePointContains::OnPoint(point),
            FacePointContains::Outside => {}
            FacePointContains::NotOnSurface => {}
        }
    }

    // choose a random point on a face
    let q = volume.all_faces()[0].inner_point();
    let geodesic = Edge::new(
        Bounds::new(other.clone(), q.clone()).unwrap(),
        Curve::Line(Line::new(other, (q - other).normalize().unwrap()).unwrap()),
    );

    let mut intersection_points = Vec::<Point>::new();
    for face in volume.all_faces().iter() {
        let intersections = curve_surface_intersection(&geodesic.curve, &*face.surface);
        match intersections {
            CurveSurfaceIntersection::Curve(_) => {
                for contour in face.boundaries.iter() {
                    match contour {
                        Contour::ContourMultiPoint(contour) => {
                            for edge in contour.edges.iter() {
                                match curve_curve_intersection(&geodesic.curve, &edge.curve) {
                                    CurveCurveIntersection::FinitePoints(points) => {
                                        for point in points {
                                            if face_point_contains(&face, point).not_outside() {
                                                intersection_points.push(point)
                                            }
                                        }
                                    }
                                    CurveCurveIntersection::InfiniteDiscretePoints(_) => todo!(),
                                    CurveCurveIntersection::Curve(_) => {
                                        if face_point_contains(&face, edge.bounds.start)
                                            .not_outside()
                                        {
                                            intersection_points.push(edge.bounds.start)
                                        }
                                        if face_point_contains(&face, edge.bounds.end).not_outside()
                                        {
                                            intersection_points.push(edge.bounds.end)
                                        }
                                    }
                                    CurveCurveIntersection::None => {}
                                }
                            }
                        }
                        Contour::ContourNoPoint(curve) => {
                            match curve_curve_intersection(&geodesic.curve, &curve.curve) {
                                CurveCurveIntersection::FinitePoints(points) => {
                                    for point in points {
                                        if face_point_contains(&face, point).not_outside() {
                                            intersection_points.push(point)
                                        }
                                    }
                                }
                                CurveCurveIntersection::InfiniteDiscretePoints(_) => todo!(),
                                CurveCurveIntersection::Curve(_) => {}
                                CurveCurveIntersection::None => {}
                            }
                        }
                    }
                }
            }
            CurveSurfaceIntersection::Points(points) => {
                for point in points {
                    match face_point_contains(&face, point) {
                        FacePointContains::Inside => intersection_points.push(point),
                        FacePointContains::OnCurveContour(_) => {
                            intersection_points.push(point);
                        }
                        FacePointContains::OnEdge(_) => {
                            intersection_points.push(point);
                        }
                        FacePointContains::OnPoint(point) => intersection_points.push(point),
                        FacePointContains::Outside => {}
                        FacePointContains::NotOnSurface => {}
                    }
                }
            }
            CurveSurfaceIntersection::None => {}
        }
    }

    // Find the closest intersection point with any other face and use the normal to determine if the point is inside or outside
    let mut closest_distance = (other - q).norm();
    let curve_dir = q - other;
    let normal = volume.boundary_normal(q);
    let mut closest_intersect_from_inside = normal.is_from_inside(curve_dir);
    for point in intersection_points.iter() {
        let distance = (other - *point).norm();
        if distance < closest_distance.lower_bound {
            let curve_dir = geodesic.curve.tangent(*point).unwrap();
            let normal = volume.boundary_normal(*point);
            closest_distance = distance;
            closest_intersect_from_inside = normal.is_from_inside(curve_dir);
        }
    }
    match closest_intersect_from_inside {
        true => VolumePointContains::Inside,
        false => VolumePointContains::Outside,
    }
}
