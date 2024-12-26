use crate::{
    curve_curve_intersection::circle_circle::{
        circle_circle_intersection, CircleCircleIntersection,
    },
    curves::{circle::Circle, CurveLike},
    point::Point,
    surface_surface_intersection::plane_sphere::{
        plane_sphere_intersection, PlaneSphereIntersection,
    },
    surfaces::{plane::Plane, sphere::Sphere},
};

pub enum CircleSphereIntersection {
    Circle(Circle),
    TwoPoints(Point, Point),
    OnePoint(Point),
    None,
}

pub fn circle_sphere_intersection(circle: &Circle, sphere: &Sphere) -> CircleSphereIntersection {
    // First find the plane that contains the circle
    let plane_circle = Plane::new(
        circle.basis,
        circle.radius,
        circle.normal.cross(circle.radius),
    );

    // Then find the intersection of the plane with the sphere
    match plane_sphere_intersection(&sphere, &plane_circle) {
        PlaneSphereIntersection::Circle(other_circle) => {
            // If the plane intersects the sphere in a circle, find the intersection of the two circles
            match circle_circle_intersection(&circle, &other_circle) {
                CircleCircleIntersection::Circle(_) => {
                    return CircleSphereIntersection::Circle(circle.clone());
                }
                CircleCircleIntersection::TwoPoint(p1, p2) => {
                    return CircleSphereIntersection::TwoPoints(p1, p2);
                }
                CircleCircleIntersection::OnePoint(p) => {
                    return CircleSphereIntersection::OnePoint(p);
                }
                CircleCircleIntersection::None => return CircleSphereIntersection::None,
            }
        }
        PlaneSphereIntersection::Point(p) => {
            if circle.on_curve(p) {
                // This is the special case where the plane touches the sphere at a single point
                return CircleSphereIntersection::OnePoint(p);
            } else {
                return CircleSphereIntersection::None;
            }
        }
        PlaneSphereIntersection::None => {
            return CircleSphereIntersection::None;
        }
    }
}

#[cfg(test)]
mod tests {
    use geop_algebra::efloat::EFloat64;

    use super::*;
    use crate::{curves::circle::Circle, point::Point, surfaces::sphere::Sphere};

    #[test]
    fn test_circle_sphere_intersection() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::zero(), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::Circle(c) => {
                assert_eq!(c.basis, Point::zero());
                assert_eq!(c.normal, Point::unit_z());
                assert_eq!(c.radius.norm(), 1.0);
            }
            _ => panic!("Expected a circle"),
        }
    }

    #[test]
    fn test_circle_sphere_intersection_two_points() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::unit_x(), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::TwoPoints(p1, p2) => {
                assert_eq!(p1, Point::from_f64(0.5, -0.8660254037844386, 0.0));
                assert_eq!(p2, Point::from_f64(0.5, 0.8660254037844386, 0.0));
            }
            _ => panic!("Expected two points"),
        }
    }

    #[test]
    fn test_circle_sphere_intersection_one_point() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::from_f64(2.0, 0.0, 0.0), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::OnePoint(p1) => {
                assert_eq!(p1, Point::from_f64(1.0, 0.0, 0.0));
            }
            _ => panic!("Expected one point"),
        }
    }

    #[test]
    fn test_circle_sphere_intersection_none() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::from_f64(10.0, 0.0, 0.0), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::None => {}
            _ => panic!("Expected None"),
        }
    }

    #[test]
    fn test_circle_sphere_intersection_one_point_tangent() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::from_f64(1.0, 0.0, 1.0), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::OnePoint(p1) => {
                assert_eq!(p1, Point::from_f64(1.0, 0.0, 0.0));
            }
            _ => panic!("Expected one point"),
        }
    }

    #[test]
    fn test_circle_sphere_intersection_none_tangent() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::from_f64(10.0, 0.0, 1.0), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::None => {}
            _ => panic!("Expected None"),
        }
    }

    #[test]
    fn test_circle_sphere_intersection_none_nowhere_close() {
        let circle = Circle::try_new(Point::zero(), Point::unit_z(), EFloat64::one()).unwrap();
        let sphere = Sphere::new(Point::from_f64(10.0, 10.0, 10.0), EFloat64::one(), true);
        match circle_sphere_intersection(&circle, &sphere) {
            CircleSphereIntersection::None => {}
            _ => panic!("Expected None"),
        }
    }
}
