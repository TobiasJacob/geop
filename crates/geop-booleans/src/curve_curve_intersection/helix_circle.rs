use crate::{
    curves::{circle::Circle, helix::Helix},
    point::Point,
};

pub enum HelixCircleIntersection {
    TwoPoints(Point, Point),
    OnePoint(Point),
    None,
}

pub fn helix_circle_intersection(helix: &Helix, circle: &Circle) -> HelixCircleIntersection {
    if helix.pitch.is_parallel(circle.normal) {
        let distance = circle.basis - helix.basis;
        let t = distance.dot(helix.pitch) / helix.pitch.norm_sq();
        let t = t.unwrap();
        let projection = distance - t * helix.pitch;
        if projection.norm() == 0.0 {
            if (circle.radius.norm() - helix.radius.norm()) == 0.0 {
                return HelixCircleIntersection::OnePoint(helix.point_at_pitch(t));
            }
        }
    }

    todo!()
}
