use crate::{surfaces::surface::Surface, curves::curve::Curve, points::point::Point};

use super::plane_plane::{plane_plane_intersection, PlanePlaneIntersection};

pub enum FaceSurfaceIntersection {
    None,
    CurvesAndPoints(Vec<Curve>, Vec<Point>),
    Surface(Surface),
}

pub fn surface_surface_intersection(
    face_self: &Surface,
    face_other: &Surface,
) -> FaceSurfaceIntersection {
    match face_self {
        Surface::Plane(plane_self) => match face_other {
            Surface::Plane(plane_other) => {
                match plane_plane_intersection(plane_self, plane_other) {
                    PlanePlaneIntersection::None => FaceSurfaceIntersection::None,
                    PlanePlaneIntersection::Line(l) => {
                        FaceSurfaceIntersection::CurvesAndPoints(vec![Curve::Line(l)], vec![])
                    }
                    PlanePlaneIntersection::Plane(p) => {
                        FaceSurfaceIntersection::Surface(Surface::Plane(p))
                    }
                }
            }
            Surface::Sphere(_sphere_other) => {
                todo!("Plane-Sphere intersection")
            }
        },
        Surface::Sphere(_sphere_self) => match face_other {
            Surface::Plane(_plane_other) => {
                todo!("Sphere-Plane intersection")
            }
            Surface::Sphere(_sphere_other) => {
                todo!("Sphere-Sphere intersection")
            }
        },
    }
}
