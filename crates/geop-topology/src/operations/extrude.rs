use std::rc::Rc;

use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
    surfaces::{plane::Plane, surface::Surface},
    transforms::Transform,
};

use crate::topology::{contour::Contour, edge::Edge, face::Face, volume::Volume};

pub fn extrude(start_face: Face, direction: Point) -> Volume {
    let end_face = 
        start_face
            .transform(Transform::from_translation(direction))
            .flip();


    let mut faces = Vec::<Face>::new();
    let all_edges = &start_face.all_edges();
    let n = all_edges.len();
    for i in 0..n {
        match all_edges[i].curve {
            Curve::Line(_) => {}
            Curve::Circle(_) => panic!("Cannot extrude circular edges"),
            Curve::Ellipse(_) => todo!(),
        }
        let top = all_edges[i].flip();
        let bottom = end_face.all_edges()[n - i - 1].flip();

        let right = Edge::new(
            bottom.end.clone(),
            top.start.clone(),
            Curve::Line(Line::new(
                bottom.end,
                top.start - bottom.end,
            )),
        );
        let left = Edge::new(
            top.end.clone(),
            bottom.start.clone(),
            Curve::Line(Line::new(top.end, bottom.start - top.end)),
        );

        let plane = Surface::Plane(Plane::new(
            bottom.start,
            bottom.end - bottom.start,
            top.end - bottom.start,
        ));
        let contour = Contour::new(vec![right, top, left, bottom]);

        let face = Face::new(vec![contour], Rc::new(plane));
        faces.push(face);
    }
    faces.push(start_face);
    faces.push(end_face);

    Volume::new(faces)
}
