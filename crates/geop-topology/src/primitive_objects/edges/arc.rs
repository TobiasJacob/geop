use geop_geometry::{
    curves::{circle::Circle, curve::Curve},
    point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_arc(from: Point, to: Point, radius: f64, normal: Point) -> Edge {
    let midpoint = (from + to) / 2.0;

    let d = to - from;

    let h = (radius * radius - d.norm_sq() / 4.0).sqrt();
    assert!(h.is_finite());
    assert!(h > 0.0);
    let center = midpoint + h * normal.cross(d).normalize().unwrap();

    Edge::new(
        Some(from),
        Some(to),
        Curve::Circle(Circle::new(center, normal, radius)),
    )
}
