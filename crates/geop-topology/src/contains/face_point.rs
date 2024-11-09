use geop_geometry::{
    curve_curve_intersection::curve_curve::{curve_curve_intersection, CurveCurveIntersection},
    curves::{curve::Curve, CurveLike},
    efloat::EFloat64,
    point::Point,
    surfaces::SurfaceLike,
};

use crate::topology::{contour::Contour, contour_no_point::ContourNoPoint, edge::Edge, face::Face};

use super::{
    contour_point::contour_point_contains,
    edge_point::{edge_point_contains, EdgePointContains},
};

#[derive(Clone, Debug)]
pub enum FacePointContains {
    Inside,
    OnContour(Contour),
    Outside,
    NotOnSurface,
}

impl FacePointContains {
    pub fn not_outside(&self) -> bool {
        match self {
            FacePointContains::Outside => false,
            _ => true,
        }
    }

    pub fn is_inside(&self) -> bool {
        match self {
            FacePointContains::Inside => true,
            _ => false,
        }
    }
}

pub fn face_point_contains(face: &Face, point: Point) -> FacePointContains {
    if !face.surface.on_surface(point) {
        return FacePointContains::NotOnSurface;
    }

    // If the point is on the border, it is part of the set
    for contour in face.boundaries.iter() {
        match contour_point_contains(contour, point) {
            EdgePointContains::Outside => {}
            _ => {
                return FacePointContains::OnContour(contour.clone());
            }
        }
    }
    // Draw a line from the point to a random point on the border.
    let q = match face.get_boundary_point() {
        Some(q) => q,
        None => {
            return FacePointContains::Inside;
        }
    };
    let geodesic = face.edge_from_to(point, q);

    // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
    let mut closest_distance = face.surface.distance(point, q);
    let curve_dir = geodesic.tangent(q);
    let normal = face.surface.normal(q);
    let contour_dir = face.boundary_tangent(q);
    let mut closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);

    let mut intersection_points = Vec::<Point>::new();
    for boundary in face.boundaries.iter() {
        match boundary {
            Contour::ContourNoPoint(_) => {
                todo!("This case should only happen when the contour is a curve");
            }
            Contour::ContourMultiPoint(boundary) => {
                for edge in boundary.edges.iter() {
                    match curve_curve_intersection(&edge.curve, &geodesic.curve) {
                        CurveCurveIntersection::FinitePoints(points) => {
                            for p in points {
                                if edge_point_contains(&geodesic, p) != EdgePointContains::Outside {
                                    if edge_point_contains(&edge, p) != EdgePointContains::Outside {
                                        intersection_points.push(p)
                                    }
                                }
                            }
                        }
                        CurveCurveIntersection::InfiniteDiscretePoints(point_array) => {
                            match geodesic.curve {
                                Curve::Helix(_) => {}
                                _ => {
                                    todo!("This case should only happen when the curve is a helix");
                                }
                            }

                            let start_i = (point - point_array.basis).dot(point_array.extend_dir);
                            let start_i = (point - point_array.basis).dot(point_array.extend_dir);
                            intersection_points
                                .push(point_array.basis + start_i * point_array.extend_dir);
                            let start_i = (point - point_array.basis).dot(point_array.extend_dir);
                            intersection_points
                                .push(point_array.basis + start_i * point_array.extend_dir);
                            intersection_points
                                .push(point_array.basis + start_i * point_array.extend_dir);
                            intersection_points.push(
                                point_array.basis
                                    + (start_i + EFloat64::one()) * point_array.extend_dir,
                            );
                            intersection_points.push(
                                point_array.basis
                                    + (start_i - EFloat64::one()) * point_array.extend_dir,
                            );
                        }
                        CurveCurveIntersection::Curve(_curve) => {
                            match edge_point_contains(&geodesic, edge.bounds.start) {
                                EdgePointContains::Inside => {
                                    intersection_points.push(edge.bounds.start)
                                }
                                EdgePointContains::Outside => {}
                                EdgePointContains::OnPoint(_) => {
                                    intersection_points.push(edge.bounds.start)
                                }
                            }
                            match edge_point_contains(&geodesic, edge.bounds.end) {
                                EdgePointContains::Inside => {
                                    intersection_points.push(edge.bounds.end)
                                }
                                EdgePointContains::Outside => {}
                                EdgePointContains::OnPoint(_) => {
                                    intersection_points.push(edge.bounds.end)
                                }
                            }
                        }
                        CurveCurveIntersection::None => {}
                    }
                }
            }
        }
    }

    for int in intersection_points {
        let distance = face.surface.distance(point, int);
        if distance < closest_distance.lower_bound {
            let curve_dir = geodesic.tangent(int);
            let normal = face.surface.normal(int);
            let contour_dir = face.boundary_tangent(int);
            closest_distance = distance;
            closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);
        }
    }

    match closest_intersect_from_inside {
        true => FacePointContains::Inside,
        false => FacePointContains::Outside,
    }
}
