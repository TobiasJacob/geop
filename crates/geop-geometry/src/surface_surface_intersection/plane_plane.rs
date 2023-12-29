use crate::{curves::line::Line, points::point::Point, surfaces::plane::Plane};

pub enum PlanePlaneIntersection {
    Plane(Plane),
    Line(Line),
    None,
}

pub fn plane_plane_intersection(a: &Plane, b: &Plane) -> PlanePlaneIntersection {
    let n_a = a.normal();
    let n_b = b.normal();
    let b_a: Point = a.basis;
    let b_b: Point = b.basis;

    let v = n_a.cross(n_b);
    if v.norm() > crate::EQ_THRESHOLD {
        let t = (n_a.dot(b_b) - n_a.dot(b_a)) / n_a.dot(v);
        PlanePlaneIntersection::Line(Line::new(b_a + v * t, v))
    } else {
        let n = n_a.dot(b_a - b_b);
        if n.abs() < crate::EQ_THRESHOLD {
            PlanePlaneIntersection::Plane(Plane::new(a.basis, a.u_slope, a.v_slope))
        } else {
            PlanePlaneIntersection::None
        }
    }
}
