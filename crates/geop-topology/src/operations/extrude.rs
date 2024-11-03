use std::rc::Rc;

use geop_geometry::{
    curves::{curve::Curve, CurveLike},
    point::Point,
    surfaces::{cylinder::Cylinder, plane::Plane, surface::Surface},
    transforms::Transform,
};

use crate::{
    primitive_objects::edges::line::primitive_line,
    topology::{
        contour::{
            connected_egde_contour::ConnectedEdgeContour, curve_contour::CurveContour, Contour,
        },
        face::Face,
        shell::Shell,
        volume::Volume,
    },
};

pub fn extrude(start_face: Face, direction: Point) -> Volume {
    let end_face = start_face
        .transform(Transform::from_translation(direction))
        .flip();

    let mut faces = Vec::<Face>::new();
    let start_boundaries = &start_face.boundaries;
    let end_boundaries = &end_face.boundaries;
    for j in 0..start_boundaries.len() {
        match (&start_boundaries[j], &end_boundaries[j]) {
            (Contour::ConnectedEdge(start_contour), Contour::ConnectedEdge(end_contour)) => {
                let start_edges = start_contour.edges.clone();
                let end_edges = end_contour.edges.clone();
                for i in 0..start_edges.len() {
                    match &start_edges[i].curve {
                        Curve::Line(line) => {
                            let top = start_edges[i].flip();
                            let bottom = end_edges
                                .iter()
                                .find(|e| {
                                    **e == start_edges[i]
                                        .transform(Transform::from_translation(direction))
                                        .flip()
                                })
                                .unwrap()
                                .flip();

                            let right =
                                primitive_line(bottom.bounds.end, top.bounds.start).unwrap();
                            let left = primitive_line(top.bounds.end, bottom.bounds.end).unwrap();

                            let plane =
                                Surface::Plane(Plane::new(line.basis, direction, line.direction));
                            let contour = Contour::from_edges(vec![right, top, left, bottom]);
                            let face = Face::new(vec![contour], Rc::new(plane));
                            faces.push(face);
                        }
                        Curve::Circle(circle) => {
                            let top = start_edges[i].flip();
                            let bottom = end_edges
                                .iter()
                                .find(|e| {
                                    **e == start_edges[i]
                                        .transform(Transform::from_translation(direction))
                                        .flip()
                                })
                                .unwrap()
                                .flip();

                            let right =
                                primitive_line(bottom.bounds.end, top.bounds.start).unwrap();
                            let left = primitive_line(top.bounds.end, bottom.bounds.end).unwrap();

                            let midpoint = circle.get_midpoint(Some(&top.bounds)).unwrap();
                            let inwards_direction =
                                direction.cross(circle.tangent(midpoint).unwrap());
                            let normal_outwards =
                                inwards_direction.dot(midpoint - circle.basis) > 0.0;

                            let cylinder = Surface::Cylinder(Cylinder::new(
                                circle.basis,
                                circle.normal,
                                circle.radius.norm(),
                                normal_outwards,
                            ));

                            let contour = Contour::from_edges(vec![right, top, left, bottom]);
                            let face = Face::new(vec![contour], Rc::new(cylinder));
                            faces.push(face);
                        }
                        Curve::Ellipse(_) => todo!("Implement this"),
                        Curve::Helix(_) => panic!("Cannot extrude helix"),
                    }
                }
            }
            (Contour::Curve(start_curve), Contour::Curve(end_curve)) => {
                match (&start_curve.curve, &end_curve.curve) {
                    (Curve::Circle(start_circle), Curve::Circle(end_circle)) => {
                        let top = start_curve.flip();
                        let bottom = end_curve.flip();

                        let midpoint = start_circle.get_midpoint(None).unwrap();
                        let inwards_direction =
                            direction.cross(start_circle.tangent(midpoint).unwrap());
                        let normal_outwards =
                            inwards_direction.dot(midpoint - start_circle.basis) > 0.0;

                        let cylinder = Surface::Cylinder(Cylinder::new(
                            start_circle.basis,
                            start_circle.normal,
                            start_circle.radius.norm(),
                            normal_outwards,
                        ));

                        let face = Face::new(
                            vec![Contour::from_curve(top), Contour::from_curve(bottom)],
                            Rc::new(cylinder),
                        );
                        faces.push(face);
                    }
                    _ => {
                        todo!("Implement this");
                    }
                }
            }
            _ => todo!("Implement this"),
        }
    }
    faces.push(start_face);
    faces.push(end_face);

    Volume::new(Shell::new(faces), vec![])
}
