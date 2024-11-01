use crate::{
    curves::{helix::Helix, line::Line, CurveLike},
    point::Point,
};

use super::curve_curve::PointArray;

pub enum HelixLineIntersection {
    PointArray(PointArray),
    TwoPoint(Point),
    OnePoint(Point),
    None,
}

pub fn helix_line_intersection(helix: &Helix, line: &Line) -> HelixLineIntersection {
    if line.direction.is_parallel(helix.pitch) {
        let distance = line.basis - helix.basis;
        let distance = distance - distance.dot(helix.pitch) * helix.pitch;
        let radius = distance.norm();
        if (radius - helix.radius.norm()) == 0.0 {
            let line_base_projected = line.basis - helix.basis;
            let line_base_projected =
                line_base_projected - line_base_projected.dot(helix.pitch) * helix.pitch;
            let angle = helix
                .radius
                .angle2(line_base_projected, helix.pitch)
                .unwrap();
            let first_point = helix.basis + helix.radius * angle.cos() + helix.pitch * angle.sin();
            assert!(helix.on_curve(first_point));
            assert!(line.on_curve(first_point));
            assert!(helix.on_curve(first_point + helix.pitch));
            assert!(line.on_curve(first_point + helix.pitch));
            let point_array = PointArray {
                basis: first_point,
                extend_dir: helix.pitch,
            };
            return HelixLineIntersection::PointArray(point_array);
        }
        return HelixLineIntersection::None;
    }
    todo!("Implement this")
}
