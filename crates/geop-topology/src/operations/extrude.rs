use std::rc::Rc;

use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
    surfaces::{plane::Plane, surface::Surface},
    transforms::Transform,
};

use crate::topology::{contour::Contour, edge::Edge, face::Face, shell::Shell, volume::Volume};

pub fn extrude(start_face: Face, direction: Point) -> Volume {
    let end_face = start_face
        .transform(Transform::from_translation(direction))
        .flip();

    let mut faces = Vec::<Face>::new();
    let all_edges = &start_face.all_edges();
    let n = all_edges.len();
    for i in 0..n {
        match &all_edges[i].curve {
            Curve::Line(line) => {
                let top = all_edges[i].flip();
                let bottom = end_face.all_edges()[n - i - 1].flip();

                let right = match (bottom.end, top.start) {
                    (Some(start), Some(end)) => Some(Edge::new(
                        Some(start),
                        Some(end),
                        Curve::Line(Line::new(start, end - start)),
                    )),
                    _ => None,
                };
                let left = match (top.end, bottom.start) {
                    (Some(start), Some(end)) => Some(Edge::new(
                        Some(start),
                        Some(end),
                        Curve::Line(Line::new(start, end - start)),
                    )),
                    _ => None,
                };

                let plane = Surface::Plane(Plane::new(line.basis, direction, line.direction));
                let contour = Contour::new(
                    vec![right, Some(top), left, Some(bottom)]
                        .drain(..)
                        .filter_map(|f| f)
                        .collect(),
                );

                let face = Face::new(contour, vec![], Rc::new(plane));
                faces.push(face);
            }
            Curve::Circle(_) => panic!("Cannot extrude circular edges"),
        }
    }
    faces.push(start_face);
    faces.push(end_face);

    Volume::new(Shell::new(faces), vec![])
}
