use std::{rc::Rc, vec};

use geop_geometry::{
    point::Point,
    surfaces::{plane::Plane, surface::Surface},
};

use crate::{
    primitive_objects::edges::line::primitive_line,
    topology::{contour::Contour, face::Face},
};

pub fn primitive_rectangle(position: Point, dir1: Point, dir2: Point) -> Face {
    let v1 = position + dir1 + dir2;
    let v2 = position - dir1 + dir2;
    let v3 = position - dir1 - dir2;
    let v4 = position + dir1 - dir2;

    Face::new(
        vec![Contour::new(vec![
            primitive_line(v1, v2).unwrap(),
            primitive_line(v2, v3).unwrap(),
            primitive_line(v3, v4).unwrap(),
            primitive_line(v4, v1).unwrap(),
        ])],
        Rc::new(Surface::Plane(Plane::new(position, dir1, dir2))),
    )
}
