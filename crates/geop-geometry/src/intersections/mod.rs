pub mod sphere_sphere;
pub mod plane_sphere;
pub mod plane_plane;
pub mod line_plane;

use crate::geometry::curves::curve3d::Curve3d;
use crate::geometry::curves::line3d::Line3d;
use crate::geometry::points::point3d::Point3d;
use crate::geometry::surfaces::sphere::Sphere;
use crate::geometry::surfaces::plane::Plane;

pub enum IntersectableSurface {
    LinearSurface(Plane),
    Sphere(Sphere),
    Line3d(Line3d)
}

pub enum IntersectableCurve3d {
    Line3d(Line3d)
}

impl IntersectableCurve3d {
    pub fn point_at(&self, u: f64) -> Point3d {
        match self {
            IntersectableCurve3d::Line3d(line) => line.point_at(u)
        }
    }

    pub fn project(&self, x: Point3d) -> f64 {
        match self {
            IntersectableCurve3d::Line3d(line) => line.project(x)
        }
    }

    // Returns a sorted list of intersections
    pub fn intersections(&self, other: &IntersectableCurve3d) -> Vec<Point3d> {
        match self {
            IntersectableCurve3d::Line3d(line) => match other {
                IntersectableCurve3d::Line3d(other_line) => {
                    todo!("Line line intersection");
                }
            }
        }
    }
}

pub fn intersect(a: IntersectableSurface, b: IntersectableSurface) {
    todo!("Intersection")

}

pub fn intersect_curve_3d(a: IntersectableCurve3d, b: IntersectableCurve3d) -> Vec<Point3d> {
    todo!("Intersection")
}