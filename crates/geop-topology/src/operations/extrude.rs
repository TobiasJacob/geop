use std::rc::Rc;

use geop_geometry::{points::point::Point, transforms::Transform, curves::line::Line, surfaces::plane::Plane};

use crate::topology::{face::{Face, FaceSurface}, volume::Volume, edge::{Edge, Direction, edge_curve::EdgeCurve}, contour::Contour};

pub fn extrude(start_face: Rc<Face>, direction: Point) -> Volume {
    let end_face = Rc::new(start_face.transform(Transform::from_translation(direction)).flip());

    let mut faces = Vec::<Rc<Face>>::new();
    let all_edges = &start_face.all_edges();
    let n = all_edges.len();
    for i in 0..n {
        match &*all_edges[i].curve {
            EdgeCurve::Line(_) => {},
            EdgeCurve::Circle(_) => panic!("Cannot extrude circular edges"),
            EdgeCurve::Ellipse(_) => todo!(),
        }
        let top = Rc::new(all_edges[i].neg());
        let bottom = Rc::new(end_face.all_edges()[n - i - 1].neg());

        let right = Rc::new(Edge::new(
            bottom.end.clone(), 
            top.start.clone(), 
            Rc::new(EdgeCurve::Line(Line::new(*bottom.end, *top.start - *bottom.end))), 
            Direction::Increasing));
        let left = Rc::new(Edge::new(
            top.end.clone(), 
            bottom.start.clone(), 
            Rc::new(EdgeCurve::Line(Line::new(*top.end, *bottom.start - *top.end))), 
            Direction::Increasing));

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