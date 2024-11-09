use crate::{curves::circle::Circle, point::Point, surfaces::cylinder::Cylinder};

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
        if distance.norm() == 0.0 {
            if (circle.radius.norm() - cylinder.radius.norm()) == 0.0 {
                return CircleCylinderIntersection::Circle(circle.clone());
            }
        }
    }

    todo!("Implement other cases")
}
