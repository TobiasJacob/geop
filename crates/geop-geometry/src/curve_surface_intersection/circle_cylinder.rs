use crate::{curves::circle::Circle, point::Point, surfaces::cylinder::Cylinder};

pub enum CircleCylinderIntersection {
    Circle(Circle),
    TwoPoints(Point, Point),
    OnePoint(Point),
    None,
}

pub fn circle_cylinder_intersection(
    circle: &Circle,
    cylinder: &Cylinder,
) -> CircleCylinderIntersection {
    if circle.normal.is_parallel(cylinder.extend_dir) {
        let distance = circle.basis - cylinder.basis;
        let distance = distance - distance.dot(cylinder.extend_dir) * cylinder.extend_dir;
        if distance.norm() == 0.0 {
            if (circle.radius.norm() - cylinder.radius.norm()) == 0.0 {
                return CircleCylinderIntersection::Circle(circle.clone());
            }
        }

        let s = distance.norm() - circle.radius.norm() - cylinder.radius.norm();
        if s > 0.0 {
            // Too far away
            return CircleCylinderIntersection::None;
        }
        // parallel circle touches the cylinder from the outside
        if s == 0.0 {
            let shared_dir = (distance / distance.norm()).unwrap();

            // Calculate touch point at cylinder's surface
            let touch_point = cylinder.basis + shared_dir * cylinder.radius.norm();

            // Add the z-coordinate from the circle's height
            let height = circle.basis.dot(cylinder.extend_dir);
            let p = touch_point + height * cylinder.extend_dir;

            return CircleCylinderIntersection::OnePoint(p);
        }
    }

    todo!("Implement other cases")
}

#[cfg(test)]
mod tests {
    use geop_algebra::efloat::EFloat64;

    use super::*;

    #[test]
    fn test_circle_cylinder_intersection_exact() {
        // test the case where the parallel circle competely intersects the cylinder if it was flat
        let cylinder = Cylinder::new(
            Point::from_f64(10.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::from(10.0),
            true,
        );

        for z_dim in 0..=10 {
            let circle = Circle::try_new(
                Point::from_f64(10.0, 0.0, z_dim as f64),
                Point::from_f64(0.0, 0.0, 1.0),
                EFloat64::from(10.0),
            )
            .unwrap();
            match circle_cylinder_intersection(&circle, &cylinder) {
                CircleCylinderIntersection::Circle(circle) => {
                    assert_eq!(circle, circle);
                }
                _ => panic!("Intersection should be a circle"),
            }
        }
    }

    #[test]
    fn test_circle_cylinder_intersection_coincident_but_parallel() {
        let cylinder = Cylinder::new(
            Point::from_f64(9.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::from(1.0),
            true,
        );

        for z_dim in 0..=10 {
            let circle = Circle::try_new(
                Point::from_f64(18.0, 0.0, z_dim as f64),
                Point::from_f64(0.0, 0.0, 1.0),
                EFloat64::from(8.0),
            )
            .unwrap();
            match circle_cylinder_intersection(&circle, &cylinder) {
                CircleCylinderIntersection::OnePoint(p) => {
                    assert_eq!(p, Point::from_f64(10.0, 0.0, z_dim as f64));
                }
                _ => panic!("Intersection should be a point"),
            }
        }
    }

    #[test]
    fn test_circle_cylinder_intersection_none_parallel() {
        let cylinder = Cylinder::new(
            Point::from_f64(9.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::from(1.0),
            true,
        );

        let circle = Circle::try_new(
            Point::from_f64(20.0, 0.0, 5.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::from(8.0),
        )
        .unwrap();
        match circle_cylinder_intersection(&circle, &cylinder) {
            CircleCylinderIntersection::None => {}
            _ => panic!("Intersection should be None"),
        }
    }
}
