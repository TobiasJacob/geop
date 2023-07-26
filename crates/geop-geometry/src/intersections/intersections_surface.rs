use crate::geometry::curves::circle3d::Circle3d;
use crate::geometry::curves::curve3d::Curve3d;
use crate::geometry::curves::line3d::Line3d;
use crate::geometry::points::point3d::Point3d;
use crate::geometry::surfaces::sphere::Sphere;
use crate::geometry::surfaces::plane::Plane;

use super::intersections_curve::IntersectableCurve3d;



pub enum IntersectableSurface {
    LinearSurface(Plane),
    Sphere(Sphere),
    Line3d(Line3d)
}

pub enum IntersectableSurfaceResult {
    IntersectableCurve3d(IntersectableCurve3d), // Ellipse3d, Circle3d, Line3d, for well defined problems.
    Plane(Plane), // e. g. 2 planes that are equal
    Sphere(Sphere), // e. g. 2 spheres that are equal
    Point3d(Point3d), // e. g. 2 spheres with distance equals to the sum of their radii
    None // e. g. 2 planes that are parallel
}

impl IntersectableSurface {
    pub fn intersect(&self, other: &IntersectableSurface) -> IntersectableSurfaceResult {
        todo!("Intersection")
    }
}
