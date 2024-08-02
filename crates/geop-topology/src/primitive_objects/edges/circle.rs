use geop_geometry::{
    curves::{circle::Circle, curve::Curve},
    points::point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_circle(basis: Point, normal: Point, radius: f64) -> Edge {
    let normal = normal.normalize();
    let c = Circle::new(basis, normal, radius);
    let start = Point::new_unit_x().cross(normal) * radius;
    Edge::new(Some(basis + start), Some(basis + start), Curve::Circle(c))
}
