use std::rc::Rc;

use geop_geometry::{
    efloat::EFloat64,
    point::Point,
    surfaces::{plane::Plane, surface::Surface},
};

use crate::{
    operations::extrude::extrude,
    primitive_objects::edges::line::primitive_line,
    topology::{contour::Contour, face::Face, volume::Volume},
};

pub fn primitive_cube(size_x: EFloat64, size_y: EFloat64, size_z: EFloat64) -> Volume {
    let p1 = Point::new(
        (-size_x / EFloat64::two()).unwrap(),
        (size_y / EFloat64::two()).unwrap(),
        (-size_z / EFloat64::two()).unwrap(),
    );
    let p2 = Point::new(
        (size_x / EFloat64::two()).unwrap(),
        (size_y / EFloat64::two()).unwrap(),
        (-size_z / EFloat64::two()).unwrap(),
    );
    let p3 = Point::new(
        (size_x / EFloat64::two()).unwrap(),
        (-size_y / EFloat64::two()).unwrap(),
        (-size_z / EFloat64::two()).unwrap(),
    );
    let p4 = Point::new(
        (-size_x / EFloat64::two()).unwrap(),
        (-size_y / EFloat64::two()).unwrap(),
        (-size_z / EFloat64::two()).unwrap(),
    );

    let edge1 = primitive_line(p1, p2).unwrap();
    let edge2 = primitive_line(p2, p3).unwrap();
    let edge3 = primitive_line(p3, p4).unwrap();
    let edge4 = primitive_line(p4, p1).unwrap();

    let face = Face::new(
        vec![Contour::from_edges(vec![
            edge1.clone(),
            edge2.clone(),
            edge3.clone(),
            edge4.clone(),
        ])],
        Rc::new(Surface::Plane(Plane::new(
            Point::new(
                EFloat64::zero(),
                EFloat64::zero(),
                (-size_z / EFloat64::two()).unwrap(),
            ),
            Point::new(EFloat64::one(), EFloat64::zero(), EFloat64::zero()),
            Point::new(EFloat64::zero(), -EFloat64::one(), EFloat64::zero()),
        ))),
    );

    extrude(face, Point::new(EFloat64::zero(), EFloat64::zero(), size_z))
}
