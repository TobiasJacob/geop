use geop_geometry::{surfaces::{sphere::{SphereTransform, Sphere}, surface::Surface, plane::Plane}, transforms::Transform, EQ_THRESHOLD, points::point::Point, surface_surface_intersection::plane_plane::{plane_plane_intersection, PlanePlaneIntersection}, curves::curve::Curve};

use crate::topology::{edge::{Edge}, intersections::edge_edge::EdgeEdgeIntersection, contour::Contour};


#[derive(PartialEq, Clone, Debug)]
pub enum FaceSurface {
    Plane(Plane),
    Sphere(Sphere),
}
impl FaceSurface {
    pub fn surface(&self) -> &dyn Surface {
        match self {
            FaceSurface::Plane(plane) => plane,
            FaceSurface::Sphere(sphere) => sphere,
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        match self {
            FaceSurface::Plane(plane) => FaceSurface::Plane(plane.transform(transform)),
            FaceSurface::Sphere(sphere) => FaceSurface::Sphere(match sphere.transform(transform) {
                SphereTransform::Ellipsoid() => panic!("Ellipsoid not implemented"),
                SphereTransform::Sphere(sphere) => sphere,
            }),
        }
    }

    pub fn contains_edge(&self, edge: &Edge) -> bool {
        if !self.surface().on_surface(*edge.start) {
            return false;
        }
        if !self.surface().on_surface(*edge.end) {
            return false;
        }
        match self {
            FaceSurface::Plane(plane) => {
                match &*edge.curve {
                    Curve::Line(line) => {
                        return plane.normal().dot(line.direction).abs() < EQ_THRESHOLD && plane.on_surface(line.basis);
                    }
                    Curve::Circle(circle) => {
                        return circle.normal.dot(plane.normal()) < EQ_THRESHOLD && plane.on_surface(circle.basis)
                    },
                    Curve::Ellipse(_) => todo!("Not implemented"),
                }
            }
            FaceSurface::Sphere(sphere) => {
                todo!("Not implemented");
            }
        }
    }

    pub fn intersect_edge(&self, other: &Edge) -> Vec<EdgeEdgeIntersection> {
        match self {
            FaceSurface::Plane(plane) => {
                match &*other.curve {
                    Curve::Line(line) => {
                        let p = todo!("asdf");
                    },
                    _ => todo!("Not implemented"),
                }
            },
            _ => todo!("Not implemented"),
        }
    }

    pub fn neg(&self) -> FaceSurface {
        match self {
            FaceSurface::Plane(plane) => FaceSurface::Plane(plane.neg()),
            FaceSurface::Sphere(sphere) => FaceSurface::Sphere(sphere.neg()),
        }
    }
}

pub enum FaceSurfaceIntersection {
    None,
    CurvesAndPoints(Vec<Curve>, Vec<Point>),
    Surface(FaceSurface),
}

pub fn face_surface_face_surface_intersect(face_self: &FaceSurface, face_other: &FaceSurface) -> FaceSurfaceIntersection {
    match face_self {
        FaceSurface::Plane(plane_self) => match face_other {
            FaceSurface::Plane(plane_other) => {
                match plane_plane_intersection(plane_self, plane_other) {
                    PlanePlaneIntersection::None => FaceSurfaceIntersection::None,
                    PlanePlaneIntersection::Line(l) => FaceSurfaceIntersection::CurvesAndPoints(vec![Curve::Line(l)], vec![]),
                    PlanePlaneIntersection::Plane(p) => FaceSurfaceIntersection::Surface(FaceSurface::Plane(p)),
                }
            },
            FaceSurface::Sphere(sphere_other) => {
                todo!("Plane-Sphere intersection")
            },
        },
        FaceSurface::Sphere(sphere_self) => match face_other {
            FaceSurface::Plane(plane_other) => {
                todo!("Sphere-Plane intersection")
            },
            FaceSurface::Sphere(sphere_other) => {
                todo!("Sphere-Sphere intersection")
            },
        },
    }
}