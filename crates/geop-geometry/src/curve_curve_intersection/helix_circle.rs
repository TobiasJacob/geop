use crate::{
    curves::{circle::Circle, helix::Helix},
    point::Point,
    EQ_THRESHOLD,
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
        let projection = distance - t * helix.pitch;
        if projection.norm() < EQ_THRESHOLD {
            if (circle.radius.norm() - helix.radius.norm()).abs() < EQ_THRESHOLD {
                return HelixCircleIntersection::OnePoint(helix.point_at_pitch(t));
            }
        }
    }

    todo!()
}
