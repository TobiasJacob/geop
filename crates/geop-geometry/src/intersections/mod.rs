pub mod sphere_sphere;
pub mod plane_sphere;
pub mod plane_plane;
pub mod line_plane;

use crate::geometry::curves::line3d::Line3d;
use crate::geometry::surfaces::sphere::Sphere;
use crate::geometry::surfaces::plane::Plane;

pub enum IntersectableSurface {
    LinearSurface(Plane),
    Sphere(Sphere),
    Line3d(Line3d)
}

pub fn intersect(a: IntersectableSurface, b: IntersectableSurface) {
    todo!("Intersection")

}
