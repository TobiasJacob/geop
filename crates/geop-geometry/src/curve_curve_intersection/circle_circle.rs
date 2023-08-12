use crate::{curves::circle::Circle, points::point::Point, EQ_THRESHOLD};

#[derive(Debug)]
pub enum CircleCircleIntersection {
    Circle(Circle),
    TwoPoint(Point, Point),
    OnePoint(Point),
    None
}

pub fn circle_circle_intersection(a: &Circle, b: &Circle) -> CircleCircleIntersection {
    let r1 = a.radius;
    let r2 = b.radius;
    let p1 = a.basis;
    let p2 = b.basis;

    let d = (p2 - p1).norm();

    if !a.normal.is_parallel(b.normal) {
        return CircleCircleIntersection::None;
    }

    if d < EQ_THRESHOLD && (r1 - r2).abs() < EQ_THRESHOLD {
        return CircleCircleIntersection::Circle(Circle::new(p1.clone(), a.normal.clone(), r1));
    }
    
    if (d - (r1 + r2)).abs() < EQ_THRESHOLD {
        let p3 = p1 + (p2 - p1) * (r1 / (r1 + r2));
        return CircleCircleIntersection::OnePoint(p3);
    }

    if d > r1 + r2 {
        return CircleCircleIntersection::None;
    }

    if d < (r1 - r2).abs() {
        return CircleCircleIntersection::None;
    }

    let a = (r1.powi(2) - r2.powi(2) + d.powi(2)) / (2.0 * d);
    let h = (r1.powi(2) - a.powi(2)).sqrt();

    let p3 = p1 + (p2 - p1) * (a / d);
    let p4 = Point::new(p3.x + h * (p2.y - p1.y) / d, p3.y - h * (p2.x - p1.x) / d, p3.z);
    let p5 = Point::new(p3.x - h * (p2.y - p1.y) / d, p3.y + h * (p2.x - p1.x) / d, p3.z);

    CircleCircleIntersection::TwoPoint(p4, p5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circle_intersection() {
        let a = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 3.0), 2.0);
        let b = Circle::new(Point::new(1.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 2.0);
        let c = Circle::new(Point::new(4.0, 0.0, 0.0), Point::new(0.0, 0.0, 2.0), 2.0);
        let d = Circle::new(Point::new(3.0, 0.0, 0.0), Point::new(0.0, 13.0, 0.0), 2.0);
        let e = Circle::new(Point::new(4.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0), 2.0);

        match circle_circle_intersection(&a, &b) {
            CircleCircleIntersection::TwoPoint(p1, p2) => {
                let i1 = Point::new(0.5, -f64::sqrt(3.75), 0.0);
                let i2 = Point::new(0.5, f64::sqrt(3.75), 0.0);
                assert_eq!(i1, p1);
                assert_eq!(i2, p2);
            },
            _ => panic!("Should be two points"),
        }

        match circle_circle_intersection(&a, &c) {
            CircleCircleIntersection::OnePoint(p1) => {
                let i1 = Point::new(2.0, 0.0, 0.0);
                assert_eq!(i1, p1);
            },
            _ => panic!("Should be one point but is {:?}", circle_circle_intersection(&a, &c)),
        }

        match circle_circle_intersection(&a, &d) {
            CircleCircleIntersection::None => {},
            _ => panic!("Should be none but is {:?}", circle_circle_intersection(&a, &d)),
        }

        match circle_circle_intersection(&c, &e) {
            CircleCircleIntersection::OnePoint(p1) => {
                let i1 = Point::new(4.0, 0.0, 0.0);
                assert_eq!(i1, p1);
            }
            _ => panic!("Should be a circle but is {:?}", circle_circle_intersection(&c, &e)),
        }


        match circle_circle_intersection(&e, &e) {
            CircleCircleIntersection::Circle(c) => {
                assert_eq!(e.basis, c.basis);
                assert_eq!(e.normal, c.normal);
                assert_eq!(e.radius, c.radius);
            },
            _ => panic!("Should be a circle but is {:?}", circle_circle_intersection(&e, &e)),
        }
    }
}