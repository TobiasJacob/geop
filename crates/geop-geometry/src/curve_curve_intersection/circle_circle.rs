use crate::{curves::circle::Circle, points::point::Point, EQ_THRESHOLD};

#[derive(Debug)]
pub enum CircleCircleIntersection {
    Circle(Circle),
    TwoPoint(Point, Point),
    OnePoint(Point),
    None,
}

pub fn circle_circle_intersection(a: &Circle, b: &Circle) -> CircleCircleIntersection {
    let radius_backup = a.radius;
    let r1 = a.radius.norm();
    let r2 = b.radius.norm();
    let p1 = a.basis;
    let p2 = b.basis;
    let n1 = a.normal;
    let n2 = b.normal;

    let d = (p1 - p2).norm();
    let a = (r1.powi(2) - r2.powi(2) + d.powi(2)) / (2.0 * d);

    // Check if both circles are on the same plane
    if n1.is_parallel(n2) && n1.is_perpendicular(p1 - p2) {
        // Check if both circles have the same centerpoint
        if d < EQ_THRESHOLD && r1 == r2 {
            return CircleCircleIntersection::Circle(Circle::new(p1, n1, radius_backup));
        }
        // Check if both circles are concentric
        else if d < EQ_THRESHOLD && r1 != r2 {
            return CircleCircleIntersection::None;
        }
        // Check if both circles intersect in one point
        else if (d - r1 - r2).abs() < EQ_THRESHOLD {
            let p = p1 + (p2 - p1).normalize() * r1;
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
    // Check if both circles are on different planes that intersect
    todo!("Implement intersection for circles on different planes");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circle_intersection() {
        let a = Circle::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 3.0),
            Point::new(2.0, 0.0, 0.0),
        );
        let b = Circle::new(
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(2.0, 0.0, 0.0),
        );
        let c: Circle = Circle::new(
            Point::new(4.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 2.0),
            Point::new(2.0, 0.0, 0.0),
        );
        let d = Circle::new(
            Point::new(6.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 2.0),
            Point::new(2.0, 0.0, 0.0),
        );

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
}
