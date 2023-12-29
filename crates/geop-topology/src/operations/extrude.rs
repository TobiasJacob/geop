use std::rc::Rc;

use geop_geometry::{points::point::Point, transforms::Transform, curves::{line::Line, curve::Curve}, surfaces::plane::Plane};

use crate::topology::{face::{Face, face_surface::FaceSurface}, volume::Volume, edge::{Edge}, contour::Contour};

pub fn extrude(start_face: Rc<Face>, direction: Point) -> Volume {
    let end_face = Rc::new(start_face.transform(Transform::from_translation(direction)).flip());

    let mut faces = Vec::<Rc<Face>>::new();
    let all_edges = &start_face.all_edges();
    let n = all_edges.len();
    for i in 0..n {
        match &*all_edges[i].curve {
            Curve::Line(_) => {},
            Curve::Circle(_) => panic!("Cannot extrude circular edges"),
            Curve::Ellipse(_) => todo!(),
        }
        let top = Rc::new(all_edges[i].neg());
        let bottom = Rc::new(end_face.all_edges()[n - i - 1].neg());

        let right = Rc::new(Edge::new(
            bottom.end.clone(), 
            top.start.clone(), 
            Rc::new(Curve::Line(Line::new(*bottom.end, *top.start - *bottom.end)))));
        let left = Rc::new(Edge::new(
            top.end.clone(), 
            bottom.start.clone(), 
            Rc::new(Curve::Line(Line::new(*top.end, *bottom.start - *top.end)))));

        let plane = FaceSurface::Plane(Plane::new(
                *bottom.start,
                *bottom.end - *bottom.start,
                *top.end - *bottom.start,
            ));
        let contour = Contour::new(vec![right, top, left, bottom]);

        let face = Face::new(
            vec![contour],
            Rc::new(plane),
        );
        faces.push(Rc::new(face));
    }
    faces.push(start_face);
    faces.push(end_face);

    Volume::new(faces)
}