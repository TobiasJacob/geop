use crate::geometry::curves::circle3d::Circle3d;
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

impl IntersectableSurface {
    pub fn intersect(&self, other: &IntersectableSurface) -> IntersectableCurve3d {
        todo!("Intersection")
    }
}
