use crate::{
    curves::circle::Circle,
    point::Point,
    surfaces::{plane::Plane, sphere::Sphere, SurfaceLike},
};

pub enum PlaneSphereIntersection {
    Circle(Circle),
    Point(Point),
    None,
}

pub fn plane_sphere_intersection(a: &Sphere, b: &Plane) -> PlaneSphereIntersection {
    // see https://math.stackexchange.com/questions/943383/determine-circle-of-intersection-of-plane-and-sphere
    let n = b.normal(b.basis).normalize().unwrap();
    let rho = (a.basis - b.basis).dot(n);
    let r = a.radius;

    if rho < r.upper_bound && rho > -r.upper_bound {
        let new_circle_center = a.basis + n * rho;
        let new_circle_radius = (r * r - rho * rho).sqrt();
        return PlaneSphereIntersection::Circle(
            Circle::try_new(new_circle_center, n, new_circle_radius.unwrap()).unwrap(),
        );
    } else if rho == r || rho == -r {
        return PlaneSphereIntersection::Point(a.basis + n * -rho);
    } else {
        return PlaneSphereIntersection::None;
    }
}

#[cfg(test)]
mod tests {
    use crate::efloat::EFloat64;

    use super::*;

    #[test]
    fn test_plane_sphere_intersection() {
        // Sphere of radius 1 centered at the origin
        let sphere = Sphere::new(Point::zero(), EFloat64::one(), true);

        // Top Plane
        let plane = Plane::new(Point::zero(), Point::unit_x(), Point::unit_y());
        let intersection = plane_sphere_intersection(&sphere, &plane);

        // Should be a circle of radius 1 centered at the origin
        match intersection {
            PlaneSphereIntersection::Circle(circle) => {
                assert_eq!(circle.basis, Point::zero());
                assert_eq!(circle.radius.norm(), 1.0);
            }
            _ => panic!("Intersection should be a circle"),
        }

        // Now move the sphere up 1 unit
        let sphere = Sphere::new(Point::unit_z(), EFloat64::one(), true);

        // Should be a single point at the origin
        match plane_sphere_intersection(&sphere, &plane) {
            PlaneSphereIntersection::Point(point) => {
                assert_eq!(point, Point::zero());
            }
            _ => panic!("Intersection should be a single point"),
        }

        // Now move the sphere down 1 unit
        let sphere = Sphere::new(Point::from_f64(0.0, 0.0, -1.0), EFloat64::one(), true);

        // Should be a single point at the origin
        match plane_sphere_intersection(&sphere, &plane) {
            PlaneSphereIntersection::Point(point) => {
                assert_eq!(point, Point::zero());
            }
            _ => panic!("Intersection should be a single point"),
        }

        // Now move the sphere in the +y direction
        let sphere = Sphere::new(Point::from_f64(0.0, 1.0, 0.0), EFloat64::one(), true);

        // Should be a circle with radius 1 centered at (0, 1, 0)
        match plane_sphere_intersection(&sphere, &plane) {
            PlaneSphereIntersection::Circle(circle) => {
                assert_eq!(circle.basis, Point::from_f64(0.0, 1.0, 0.0));
                assert_eq!(circle.radius.norm(), 1.0);
            }
            _ => panic!("Intersection should be a circle"),
        }

        // Move the sphere far enough that there is no intersection
        let sphere = Sphere::new(Point::from_f64(1.0, 1.0, 5.0), EFloat64::one(), true);

        // Should be no intersection
        match plane_sphere_intersection(&sphere, &plane) {
            PlaneSphereIntersection::None => (),
            _ => panic!("Intersection should be no intersection"),
        }
    }
}
