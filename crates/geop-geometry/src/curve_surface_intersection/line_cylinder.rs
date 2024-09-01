use crate::{curves::line::Line, points::point::Point, surfaces::cylinder::Cylinder};

pub enum CylinderLineIntersection {
    Line(Line),
    TwoPoints(Point, Point),
    Point(Point),
    None,
}

pub fn line_cylinder_intersection(line: &Line, cylinder: &Cylinder) -> CylinderLineIntersection {
    if cylinder.extend_dir.is_parallel(line.direction) {
        let distance = line.basis - cylinder.basis;
        let distance = distance - distance.dot(cylinder.extend_dir) * cylinder.extend_dir;
        let radius = distance.norm();
        if (radius - cylinder.radius.norm()).abs() < f64::EPSILON {
            return CylinderLineIntersection::Line(line.clone());
        }
        return CylinderLineIntersection::None;
    }
    todo!("Implement other cases")
}
