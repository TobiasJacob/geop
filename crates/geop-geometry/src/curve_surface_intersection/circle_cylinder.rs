use crate::{
    curves::circle::Circle, points::point::Point, surfaces::cylinder::Cylinder, EQ_THRESHOLD,
};

pub enum CircleCylinderIntersection {
    Circle(Circle),
    TwoPoints(Point, Point),
    OnePoint(Point),
    None,
}

pub fn circle_cylinder_intersection(
    circle: &Circle,
    cylinder: &Cylinder,
) -> CircleCylinderIntersection {
    if circle.normal.is_parallel(cylinder.extend_dir) {
        let distance = circle.basis - cylinder.basis;
        let distance = distance - distance.dot(cylinder.extend_dir) * cylinder.extend_dir;
        if distance.norm() < EQ_THRESHOLD {
            if (circle.radius.norm() - cylinder.radius.norm()).abs() < f64::EPSILON {
                return CircleCylinderIntersection::Circle(circle.clone());
            }
        }
    }

    todo!("Implement other cases")
}
