use std::rc::Rc;

use geop_geometry::{
    point::Point,
    surfaces::{sphere::Sphere, surface::Surface},
};

use crate::topology::face::Face;

pub fn primitive_sphere(basis: Point, radius: f64) -> Face {
    let sphere = Sphere::new(basis, radius, true);
    Face::new(vec![], Rc::new(Surface::Sphere(sphere)))
}
