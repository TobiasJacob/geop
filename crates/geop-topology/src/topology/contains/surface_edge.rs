use geop_geometry::{curves::curve::Curve, surfaces::surface::Surface, EQ_THRESHOLD};

use crate::topology::edge::Edge;

pub fn surface_edge_contains(surface: &Surface, edge: &Edge) -> bool {
    if !surface.on_surface(edge.start) {
        return false;
    }
    if !surface.on_surface(edge.end) {
        return false;
    }
    match surface {
        Surface::Plane(plane) => match &edge.curve {
            Curve::Line(line) => {
                return plane.normal().dot(line.direction).abs() < EQ_THRESHOLD
                    && plane.on_surface(line.basis);
            }
            Curve::Circle(circle) => {
                return circle.normal.dot(plane.normal()) < EQ_THRESHOLD
                    && plane.on_surface(circle.basis)
            }
            Curve::Ellipse(_) => todo!("Not implemented"),
        },
        Surface::Sphere(_sphere) => {
            todo!("Not implemented");
        }
    }
}
