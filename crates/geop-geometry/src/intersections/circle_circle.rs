use crate::{geometry::{curves::circle::Circle, points::point::Point}, EQ_THRESHOLD};

pub enum CircleCircleIntersection {
    Circle(Circle),
    TwoPoint(Point, Point),
    SinglePoint(Point),
    None
}

pub fn circle_circle_intersection(a: &Circle, b: &Circle) -> CircleCircleIntersection {
    let r1 = a.radius;
    let r2 = b.radius;
    let p1 = a.basis;
    let p2 = b.basis;

    let d = (p2 - p1).norm();

    if d < EQ_THRESHOLD && (r1 - r2).abs() < EQ_THRESHOLD {
        return CircleCircleIntersection::Circle(Circle::new(p1, a.normal, r1));
    }
    
    if (d - (r1 + r2)).abs() < EQ_THRESHOLD {
        let p3 = p1 + (p2 - p1) * (r1 / (r1 + r2));
        return CircleCircleIntersection::SinglePoint(p3);
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
