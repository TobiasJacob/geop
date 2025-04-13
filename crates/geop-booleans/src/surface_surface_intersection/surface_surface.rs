use crate::{curves::curve::Curve, point::Point, surfaces::surface::Surface};

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
            Surface::Cylinder(_cylinder_other) => {
                todo!("Plane-Cylinder intersection")
            }
        },
        Surface::Sphere(_sphere_self) => match face_other {
            Surface::Plane(_plane_other) => {
                todo!("Sphere-Plane intersection")
            }
            Surface::Sphere(_sphere_other) => {
                todo!("Sphere-Sphere intersection")
            }
            Surface::Cylinder(_cylinder_other) => {
                todo!("Sphere-Cylinder intersection")
            }
        },
        Surface::Cylinder(_cylinder_self) => match face_other {
            Surface::Plane(_plane_other) => {
                todo!("Cylinder-Plane intersection")
            }
            Surface::Sphere(_sphere_other) => {
                todo!("Cylinder-Sphere intersection")
            }
            Surface::Cylinder(_cylinder_other) => {
                todo!("Cylinder-Cylinder intersection")
            }
        },
    }
}
