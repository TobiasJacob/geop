use crate::{curves::line::Line, points::point::Point, surfaces::sphere::Sphere, EQ_THRESHOLD};

pub enum LineSphereIntersection {
    TwoPoints(Point, Point),
    OnePoint(Point),
    None,
}

pub fn line_sphere_intersection(line: &Line, sphere: &Sphere) -> LineSphereIntersection {
    let r: f64 = sphere.radius;
    let b: Point = sphere.basis;
    let a: Point = line.basis;
    let v: Point = line.direction;

    let discriminant = 4.0 * (v.dot(a - b)).powi(2)
        - 4.0 * (v.norm().powi(2)) * ((a - b).norm().powi(2) - r.powi(2));

    if discriminant > EQ_THRESHOLD {
        let t1 = (-2.0 * v.dot(a - b) + discriminant.sqrt()) / (2.0 * v.norm().powi(2));
        let t2 = (-2.0 * v.dot(a - b) - discriminant.sqrt()) / (2.0 * v.norm().powi(2));
        LineSphereIntersection::TwoPoints(a + v * t1, a + v * t2)
    } else if discriminant <= EQ_THRESHOLD && discriminant >= -EQ_THRESHOLD {
        let t = (-2.0 * v.dot(a - b)) / (2.0 * v.norm().powi(2));
        LineSphereIntersection::OnePoint(a + v * t)
    } else {
        LineSphereIntersection::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{curves::line::Line, points::point::Point, surfaces::sphere::Sphere};

    #[test]
    fn test_line_sphere_intersection_two_points() {
        let line = Line::new(Point::zero(), Point::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(Point::zero(), 3.0, true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::new(0.0, 0.0, 3.0));
                assert_eq!(p2, Point::new(0.0, 0.0, -3.0));
            }
            _ => {
                panic!("Should intersect at two points");
            }
        }

        let line_2 = Line::new(Point::new(0.5, 0.5, 0.5), Point::new(0.0, 0.0, 1.0));
        match line_sphere_intersection(&line_2, &sphere) {
            LineSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::new(0.5, 0.5, 2.9154759474226504));
                assert_eq!(p2, Point::new(0.5, 0.5, -2.9154759474226504));
            }
            _ => {
                panic!("Should intersect at two points");
            }
        }

        let line_3 = Line::new(Point::new(0.5, -10.0, 0.5), Point::new(0.0, 1.0, 0.0));
        match line_sphere_intersection(&line_3, &sphere) {
            LineSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::new(0.5, 2.9154759474226504, 0.5));
                assert_eq!(p2, Point::new(0.5, -2.9154759474226504, 0.5));
            }
            _ => {
                panic!("Should intersect at two points");
            }
        }
    }

    #[test]
    fn test_line_sphere_intersection_one_point() {
        let line = Line::new(Point::new(1.0, 10.0, 0.0), Point::new(0.0, 1.0, 0.0));
        let sphere = Sphere::new(Point::zero(), 1.0, true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::OnePoint(p1) => {
                assert_eq!(p1, Point::new(1.0, 0.0, 0.0));
            }
            _ => {
                panic!("Should intersect at one point");
            }
        }

        let line = Line::new(Point::new(0.0, 0.0, 1.0), Point::new(1.0, 1.0, 0.0));
        let sphere = Sphere::new(Point::zero(), 1.0, true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::OnePoint(p1) => {
                assert_eq!(p1, Point::new(0.0, 0.0, 1.0));
            }
            _ => {
                panic!("Should intersect at one point");
            }
        }
    }

    #[test]
    fn test_line_sphere_intersection_none() {
        let line = Line::new(Point::new(10.0, 10.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let sphere = Sphere::new(Point::zero(), 1.0, true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::None => {}
            _ => {
                panic!("Should not intersect");
            }
        }
    }
}
