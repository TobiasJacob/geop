use geop_algebra::efloat::EFloat64;

use crate::{curves::line::Line, point::Point, surfaces::sphere::Sphere};

pub enum LineSphereIntersection {
    TwoPoints(Point, Point),
    OnePoint(Point),
    None,
}

pub fn line_sphere_intersection(line: &Line, sphere: &Sphere) -> LineSphereIntersection {
    let r: EFloat64 = sphere.radius;
    let b: Point = sphere.basis;
    let a: Point = line.basis;
    let v: Point = line.direction;

    let discriminant = EFloat64::from(4.0) * (v.dot(a - b)).powi(2)
        - EFloat64::from(4.0) * (v.norm().powi(2)) * ((a - b).norm().powi(2) - r.powi(2));

    if discriminant > 0.0 {
        let t1 = (-EFloat64::two() * v.dot(a - b) + discriminant.sqrt().unwrap())
            / (EFloat64::two() * v.norm().powi(2));
        let t2 = (-EFloat64::two() * v.dot(a - b) - discriminant.sqrt().unwrap())
            / (EFloat64::two() * v.norm().powi(2));
        let t1 = t1.unwrap();
        let t2 = t2.unwrap();
        LineSphereIntersection::TwoPoints(a + v * t1, a + v * t2)
    } else if discriminant == 0.0 {
        let t = (-EFloat64::two() * v.dot(a - b)) / (EFloat64::two() * v.norm().powi(2));
        let t = t.unwrap();
        LineSphereIntersection::OnePoint(a + v * t)
    } else {
        LineSphereIntersection::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{curves::line::Line, point::Point, surfaces::sphere::Sphere};

    #[test]
    fn test_line_sphere_intersection_two_points() {
        let line = Line::new(Point::zero(), Point::from_f64(0.0, 0.0, 1.0)).unwrap();
        let sphere = Sphere::new(Point::zero(), EFloat64::from(3.0), true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::from_f64(0.0, 0.0, 3.0));
                assert_eq!(p2, Point::from_f64(0.0, 0.0, -3.0));
            }
            _ => {
                panic!("Should intersect at two points");
            }
        }

        let line_2 = Line::new(
            Point::from_f64(0.5, 0.5, 0.5),
            Point::from_f64(0.0, 0.0, 1.0),
        )
        .unwrap();
        match line_sphere_intersection(&line_2, &sphere) {
            LineSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::from_f64(0.5, 0.5, 2.9154759474226504));
                assert_eq!(p2, Point::from_f64(0.5, 0.5, -2.9154759474226504));
            }
            _ => {
                panic!("Should intersect at two points");
            }
        }

        let line_3 = Line::new(
            Point::from_f64(0.5, -10.0, 0.5),
            Point::from_f64(0.0, 1.0, 0.0),
        )
        .unwrap();
        match line_sphere_intersection(&line_3, &sphere) {
            LineSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::from_f64(0.5, 2.9154759474226504, 0.5));
                assert_eq!(p2, Point::from_f64(0.5, -2.9154759474226504, 0.5));
            }
            _ => {
                panic!("Should intersect at two points");
            }
        }
    }

    #[test]
    fn test_line_sphere_intersection_one_point() {
        let line = Line::new(
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
        )
        .unwrap();
        let sphere = Sphere::new(Point::zero(), EFloat64::from(1.0), true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::OnePoint(p1) => {
                assert_eq!(p1, Point::from_f64(1.0, 0.0, 0.0));
            }
            _ => {
                panic!("Should intersect at one point");
            }
        }

        let line = Line::new(
            Point::from_f64(0.0, 0.0, 1.0),
            Point::from_f64(1.0, 1.0, 0.0).normalize().unwrap(),
        )
        .unwrap();
        let sphere = Sphere::new(Point::zero(), EFloat64::one(), true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::OnePoint(p1) => {
                assert_eq!(p1, Point::from_f64(0.0, 0.0, 1.0));
            }
            _ => {
                panic!("Should intersect at one point");
            }
        }
    }

    #[test]
    fn test_line_sphere_intersection_none() {
        let line = Line::new(
            Point::from_f64(10.0, 10.0, 0.0),
            Point::from_f64(1.0, 1.0, 1.0).normalize().unwrap(),
        )
        .unwrap();
        let sphere = Sphere::new(Point::zero(), EFloat64::one(), true);
        match line_sphere_intersection(&line, &sphere) {
            LineSphereIntersection::None => {}
            _ => {
                panic!("Should not intersect");
            }
        }
    }
}
