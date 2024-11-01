use std::rc::Rc;

use geop_geometry::{
    curves::{curve::Curve, line::Line, CurveLike},
    point::Point,
    surfaces::{cylinder::Cylinder, plane::Plane, surface::Surface},
    transforms::Transform,
};

use crate::topology::{contour::Contour, edge::Edge, face::Face, shell::Shell, volume::Volume};

pub fn extrude(start_face: Face, direction: Point) -> Volume {
    let end_face = start_face
        .transform(Transform::from_translation(direction))
        .flip();

    let mut faces = Vec::<Face>::new();
    let all_edges = &start_face.all_edges();
    let end_edges = &end_face.all_edges();
    let n = all_edges.len();
    for i in 0..n {
        match &all_edges[i].curve {
            Curve::Line(line) => {
                let top = all_edges[i].flip();
                let bottom = end_edges
                    .iter()
                    .find(|e| {
                        **e == all_edges[i]
                            .transform(Transform::from_translation(direction))
                            .flip()
                    })
                    .unwrap()
                    .flip();

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

                let face = Face::new(vec![contour], Rc::new(plane));
                faces.push(face);
            }
            Curve::Circle(circle) => {
                let top = all_edges[i].flip();
                let bottom = end_edges
                    .iter()
                    .find(|e| {
                        **e == all_edges[i]
                            .transform(Transform::from_translation(direction))
                            .flip()
                    })
                    .unwrap()
                    .flip();

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

                let midpoint = circle.get_midpoint(top.start, top.end);
                let inwards_direction = direction.cross(circle.tangent(midpoint));
                let normal_outwards = inwards_direction.dot(midpoint - circle.basis) > 0.0;

                let cylinder = Surface::Cylinder(Cylinder::new(
                    circle.basis,
                    circle.normal,
                    circle.radius.norm(),
                    normal_outwards,
                ));

                match (left, right) {
                    (Some(left), Some(right)) => {
                        let contour = Contour::new(vec![right, top, left, bottom]);

                        let face = Face::new(vec![contour], Rc::new(cylinder));
                        faces.push(face);
                    }
                    (None, None) => {
                        let contour = Contour::new(vec![top]);

                        let face =
                            Face::new(vec![contour, Contour::new(vec![bottom])], Rc::new(cylinder));
                        faces.push(face);
                    }
                    _ => todo!("Not implemented"),
                }
            }
            Curve::Ellipse(_) => todo!("Implement this"),
            Curve::Helix(_) => panic!("Cannot extrude helix"),
        }
    }
    faces.push(start_face);
    faces.push(end_face);

    Volume::new(Shell::new(faces), vec![])
}
