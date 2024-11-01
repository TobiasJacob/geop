use std::rc::Rc;

use geop_geometry::{
    point::Point,
    surfaces::{plane::Plane, surface::Surface},
};

use crate::{
    operations::extrude::extrude,
    primitive_objects::edges::line::primitive_line,
    topology::{contour::Contour, face::Face, volume::Volume},
};

pub fn primitive_cube(size_x: f64, size_y: f64, size_z: f64) -> Volume {
    let p1 = Point::new(-size_x / 2.0, size_y / 2.0, -size_z / 2.0);
    let p2 = Point::new(size_x / 2.0, size_y / 2.0, -size_z / 2.0);
    let p3 = Point::new(size_x / 2.0, -size_y / 2.0, -size_z / 2.0);
    let p4 = Point::new(-size_x / 2.0, -size_y / 2.0, -size_z / 2.0);

    let edge1 = primitive_line(p1, p2);
    let edge2 = primitive_line(p2, p3);
    let edge3 = primitive_line(p3, p4);
    let edge4 = primitive_line(p4, p1);

    let face = Face::new(
        vec![Contour::new(vec![
            edge1.clone(),
            edge2.clone(),
            edge3.clone(),
            edge4.clone(),
        ])],
        Rc::new(Surface::Plane(Plane::new(
            Point::new(0.0, 0.0, -size_z / 2.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, -1.0, 0.0),
        ))),
    );

    extrude(face, Point::new(0.0, 0.0, size_z))
}
