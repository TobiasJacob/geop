use crate::{
    curve_surface_intersection::circle_plane::{
        circle_plane_intersection, CirclePlaneIntersection,
    },
    curves::{circle::Circle, CurveLike},
    point::Point,
    surfaces::plane::Plane,
    EQ_THRESHOLD,
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
    let a = (r1.powi(2) - r2.powi(2) + d.powi(2)) / (2.0 * d);

    // Check if both circles are on the same plane
    if n1.is_parallel(n2) && n1.is_perpendicular(p1 - p2) {
        // Check if both circles have the same centerpoint
        if d < EQ_THRESHOLD && (r1 - r2).abs() < EQ_THRESHOLD {
            return CircleCircleIntersection::Circle(Circle::new(p1, n1, radius_backup.norm()));
        }
        // Check if both circles are concentric
        else if d < EQ_THRESHOLD && r1 != r2 {
            return CircleCircleIntersection::None;
        }
        // Check if both circles intersect in one point from outside
        else if (d - r1 - r2).abs() < EQ_THRESHOLD {
            let p = p1 + (p2 - p1).normalize() * r1;
            return CircleCircleIntersection::OnePoint(p);
        }
        // Check if both circles intersect in one point from inside
        else if (r1 - d - r2).abs() < EQ_THRESHOLD {
            let p = p1 + (p2 - p1).normalize() * r1;
            return CircleCircleIntersection::OnePoint(p);
        } else if (r2 - d - r1).abs() < EQ_THRESHOLD {
            let p = p2 + (p1 - p2).normalize() * r2;
            return CircleCircleIntersection::OnePoint(p);
        }
        // Check if both circles are disjoint
        else if d > r1 + r2 {
            return CircleCircleIntersection::None;
        }
        // Check if two point intersection
        else {
            let p = p1 + (p2 - p1).normalize() * a;
            let h = (r1.powi(2) - a.powi(2)).sqrt();
            let n = (p2 - p1).normalize().cross(n1).normalize();
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
        let a = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 3.0), 2.0);
        let b = Circle::new(Point::new(1.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 2.0);
        let c: Circle = Circle::new(Point::new(4.0, 0.0, 0.0), Point::new(0.0, 0.0, 2.0), 2.0);
        let d = Circle::new(Point::new(6.0, 0.0, 0.0), Point::new(0.0, 0.0, 2.0), 2.0);

        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::TwoPoint(p1, p2) => {
                let i1 = Point::new(0.5, -f64::sqrt(3.75), 0.0);
                let i2 = Point::new(0.5, f64::sqrt(3.75), 0.0);
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
                let i1 = Point::new(2.0, 0.0, 0.0);
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
        let a = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        let b = Circle::new(Point::new(1.0, 0.0, 2.0), Point::new(-1.0, 0.0, 0.0), 1.0);
        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::None => {}
            _ => panic!("Should be None!"),
        }

        let a = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        let b = Circle::new(Point::new(1.0, 0.0, 1.0), Point::new(-1.0, 0.0, 0.0), 1.0);
        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::OnePoint(p) => {
                assert_eq!(p, Point::new(1.0, 0.0, 0.0));
            }
            _ => panic!("Should be one point!"),
        }

        let a = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        let b = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(-1.0, 0.0, 0.0), 1.0);
        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::TwoPoint(p1, p2) => {
                assert_eq!(p1, Point::new(0.0, -1.0, 0.0));
                assert_eq!(p2, Point::new(0.0, 1.0, 0.0));
            }
            _ => panic!("Should be two points!"),
        }
    }
}
