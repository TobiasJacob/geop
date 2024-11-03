use geop_geometry::{
    curves::{bounds::Bounds, circle::Circle, curve::Curve},
    efloat::EFloat64,
    point::Point,
};

use crate::topology::edge::Edge;

pub fn primitive_arc(from: Point, to: Point, radius: EFloat64, normal: Point) -> Edge {
    let midpoint = ((from + to) / EFloat64::two()).unwrap();

    let d = to - from;

    let h = (radius * radius - (d.norm_sq() / EFloat64::from(4.0)).unwrap())
        .sqrt()
        .unwrap();
    assert!(h > 0.0);
    let center = midpoint + h * normal.cross(d).normalize().unwrap();

    Edge::new(
        Bounds::new(from, to).unwrap(),
        Curve::Circle(Circle::new(center, normal, radius)),
    )
}
