use std::rc::Rc;

use geop_geometry::{points::point::Point, transforms::Transform, curves::line::Line, surfaces::plane::Plane};

use crate::topology::{face::{Face, FaceSurface}, object::Object, edge::{Edge, Direction, EdgeCurve}, contour::Contour};

pub fn extrude(start_face: Rc<Face>, direction: Point) -> Object {
    let end_face = Rc::new(start_face.transform(Transform::from_translation(direction)).neg());

    let mut faces = Vec::<Rc<Face>>::new();
    let n = start_face.all_edges().len();
    for i in 0..n {
        let bottom = start_face.all_edges()[i].clone();
        let top = end_face.all_edges()[n - i].clone();

        let right = Rc::new(Edge::new(
            bottom.end.clone(), 
            top.start.clone(), 
            Rc::new(EdgeCurve::Line(Line::new(*bottom.end.point, *top.start.point - *bottom.end.point))), 
            Direction::Increasing));
        let left = Rc::new(Edge::new(
            top.end.clone(), 
            bottom.start.clone(), 
            Rc::new(EdgeCurve::Line(Line::new(*top.end.point, *bottom.start.point - *top.end.point))), 
            Direction::Increasing));

        let plane = FaceSurface::Plane(Plane::new(
                *bottom.start.point,
                *bottom.end.point - *bottom.start.point,
                *top.end.point - *top.start.point,
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

    Object::new(faces)
}