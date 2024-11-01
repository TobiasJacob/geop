use crate::{
    curve_surface_intersection::circle_plane::{
        circle_plane_intersection, CirclePlaneIntersection,
    },
    curves::{circle::Circle, CurveLike},
    efloat::EFloat64,
    point::Point,
    surfaces::plane::Plane,
};

#[derive(Debug)]
pub enum CircleCircleIntersection {
    Circle(Circle),
    TwoPoint(Point, Point),
    OnePoint(Point),
    None,
}

pub fn circle_circle_intersection(
    circle_a: &Circle,
    circle_b: &Circle,
) -> CircleCircleIntersection {
    let radius_backup = circle_a.radius;
    let r1 = circle_a.radius.norm();
    let r2 = circle_b.radius.norm();
    let p1 = circle_a.basis;
    let p2 = circle_b.basis;
    let n1 = circle_a.normal;
    let n2 = circle_b.normal;

    let d = (p1 - p2).norm();
    let a = (r1 * r1 - r2 * r2 + d * d) / (EFloat64::two() * d);

    // Check if both circles are on the same plane
    if n1.is_parallel(n2) && n1.is_perpendicular(p1 - p2) {
        // Check if both circles have the same centerpoint
        if d == 0.0 && (r1 - r2) == 0.0 {
            return CircleCircleIntersection::Circle(Circle::new(p1, n1, radius_backup.norm()));
        }
        // Check if both circles are concentric
        else if d == 0.0 && r1 != r2 {
            return CircleCircleIntersection::None;
        }
        // Check if both circles intersect in one point from outside
        else if (d - r1 - r2) == 0.0 {
            let p = p1 + (p2 - p1).normalize().unwrap() * r1;
            return CircleCircleIntersection::OnePoint(p);
        }
        // Check if both circles intersect in one point from inside
        else if (r1 - d - r2) == 0.0 {
            let p = p1 + (p2 - p1).normalize().unwrap() * r1;
            return CircleCircleIntersection::OnePoint(p);
        } else if (r2 - d - r1) == 0.0 {
            let p = p2 + (p1 - p2).normalize().unwrap() * r2;
            return CircleCircleIntersection::OnePoint(p);
        }
        // Check if both circles are disjoint
        else if d > (r1 + r2).lower_bound {
            return CircleCircleIntersection::None;
        }
        // Check if two point intersection
        else {
            let a = a.unwrap();
            let p = p1 + (p2 - p1).normalize().unwrap() * a;
            let h = (r1 * r1 - a * a).sqrt();
            let h = h.unwrap();
            let n = (p2 - p1)
                .normalize()
                .unwrap()
                .cross(n1)
                .normalize()
                .unwrap();
            let p1 = p + n * h;
            let p2 = p - n * h;
            return CircleCircleIntersection::TwoPoint(p1, p2);
        }
    }

    // Parallel but different planes
    if n1.is_parallel(n2) {
        return CircleCircleIntersection::None;
    }

    // Okay, the circles are on different planes. Do they intersect?
    // First find the plane that contains the second circle
    let circle_b_plane = Plane::new(
        circle_b.basis,
        circle_b.radius,
        circle_b.normal.cross(circle_b.radius),
    );
    match circle_plane_intersection(&circle_a, &circle_b_plane) {
        CirclePlaneIntersection::Circle(_) => {
            panic!("This should not happen!");
        }
        CirclePlaneIntersection::None => {
            return CircleCircleIntersection::None;
        }
        CirclePlaneIntersection::OnePoint(point) => {
            if circle_b.on_curve(point) {
                return CircleCircleIntersection::OnePoint(point);
            } else {
                return CircleCircleIntersection::None;
            }
        }
        CirclePlaneIntersection::TwoPoints(p1, p2) => {
            if circle_b.on_curve(p1) {
                if circle_b.on_curve(p2) {
                    return CircleCircleIntersection::TwoPoint(p1, p2);
                } else {
                    return CircleCircleIntersection::OnePoint(p1);
                }
            } else if circle_b.on_curve(p2) {
                return CircleCircleIntersection::OnePoint(p2);
            } else {
                return CircleCircleIntersection::None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circle_intersection() {
        let a = Circle::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 3.0).normalize().unwrap(),
            EFloat64::two(),
        );
        let b = Circle::new(
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0).normalize().unwrap(),
            EFloat64::two(),
        );
        let c: Circle = Circle::new(
            Point::from_f64(4.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 2.0).normalize().unwrap(),
            EFloat64::two(),
        );
        let d = Circle::new(
            Point::from_f64(6.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 2.0).normalize().unwrap(),
            EFloat64::two(),
        );

        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::TwoPoint(p1, p2) => {
                let i1 = Point::from_f64(0.5, -f64::sqrt(3.75), 0.0);
                let i2 = Point::from_f64(0.5, f64::sqrt(3.75), 0.0);
                assert_eq!(p1, i1);
                assert_eq!(p2, i2);
            }
            _ => panic!(
                "Should be two points but is {:?}",
                circle_circle_intersection(&a, &b)
            ),
        }

        match circle_circle_intersection(&a, &c) {
            CircleCircleIntersection::OnePoint(p1) => {
                let i1 = Point::from_f64(2.0, 0.0, 0.0);
                assert_eq!(p1, i1);
            }
            _ => panic!(
                "Should be one point but is {:?}",
                circle_circle_intersection(&a, &c)
            ),
        }

        match circle_circle_intersection(&a, &d) {
            CircleCircleIntersection::None => {}
            _ => panic!(
                "Should be none but is {:?}",
                circle_circle_intersection(&a, &d)
            ),
        }

        match circle_circle_intersection(&a, &a) {
            CircleCircleIntersection::Circle(c) => {
                assert_eq!(c, a);
            }
            _ => panic!(
                "Should be a circle but is {:?}",
                circle_circle_intersection(&a, &a)
            ),
        }
    }

    #[test]
    fn test_circle_circle_intersection_not_coplanar() {
        let a = Circle::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::one(),
        );
        let b = Circle::new(
            Point::from_f64(1.0, 0.0, 2.0),
            Point::from_f64(-1.0, 0.0, 0.0),
            EFloat64::one(),
        );
        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::None => {}
            _ => panic!("Should be None!"),
        }

        let a = Circle::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::one(),
        );
        let b = Circle::new(
            Point::from_f64(1.0, 0.0, 1.0),
            Point::from_f64(-1.0, 0.0, 0.0),
            EFloat64::one(),
        );
        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::OnePoint(p) => {
                assert_eq!(p, Point::from_f64(1.0, 0.0, 0.0));
            }
            _ => panic!("Should be one point!"),
        }

        let a = Circle::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::one(),
        );
        let b = Circle::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(-1.0, 0.0, 0.0),
            EFloat64::one(),
        );
        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::TwoPoint(p1, p2) => {
                assert_eq!(p1, Point::from_f64(0.0, -1.0, 0.0));
                assert_eq!(p2, Point::from_f64(0.0, 1.0, 0.0));
            }
            _ => panic!("Should be two points!"),
        }
    }
}
