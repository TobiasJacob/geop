use crate::{
    curve_curve_intersection::circle_line::{circle_line_intersection, CircleLineIntersection},
    curves::circle::Circle,
    point::Point,
    surface_surface_intersection::plane_plane::{plane_plane_intersection, PlanePlaneIntersection},
    surfaces::plane::Plane,
};

pub enum CirclePlaneIntersection {
    None,
    TwoPoints(Point, Point),
    OnePoint(Point),
    Circle(Circle),
}

pub fn circle_plane_intersection(circle: &Circle, plane: &Plane) -> CirclePlaneIntersection {
    // First find the plane that contains the circle
    let plane_circle = Plane::new(
        circle.basis,
        circle.radius,
        circle.normal.cross(circle.radius),
    );

    // Then find the intersection of the two planes
    match plane_plane_intersection(&plane, &plane_circle) {
        PlanePlaneIntersection::Plane(_plane) => {
            return CirclePlaneIntersection::Circle(circle.clone());
        }
        PlanePlaneIntersection::None => {
            return CirclePlaneIntersection::None;
        }
        PlanePlaneIntersection::Line(line) => {
            // If the planes intersect in a line, find the intersection of the circle with that line
            match circle_line_intersection(&circle, &line) {
                CircleLineIntersection::TwoPoint(p1, p2) => {
                    return CirclePlaneIntersection::TwoPoints(p1, p2);
                }
                CircleLineIntersection::OnePoint(p) => {
                    return CirclePlaneIntersection::OnePoint(p);
                }
                CircleLineIntersection::None => return CirclePlaneIntersection::None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::efloat::EFloat64;

    use super::*;

    #[test]
    fn test_circle_plane_intersection_complete() {
        // test the case where the circle lies completely on the plane
        let circle = Circle::try_new(
            Point::from_f64(0.5, 0.5, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
            EFloat64::two(),
        )
        .unwrap();

        let plane = Plane::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );

        match circle_plane_intersection(&circle, &plane) {
            CirclePlaneIntersection::Circle(circle) => {
                assert_eq!(circle, circle);
            }
            _ => panic!("Intersection should be a circle"),
        }
    }

    #[test]
    fn test_circle_plane_intersection_tangent() {
        // test the case where the circle is tangent to the plane, so one intersection point
        let circle = Circle::try_new(
            Point::from_f64(0.0, 0.0, -1.0),
            Point::from_f64(0.0, 1.0, 0.0),
            EFloat64::one(),
        )
        .unwrap();

        let plane = Plane::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );

        match circle_plane_intersection(&circle, &plane) {
            CirclePlaneIntersection::OnePoint(point) => {
                assert_eq!(point, Point::from_f64(0.0, 0.0, 0.0));
            }
            _ => panic!("Intersection should be a single point"),
        }
    }
}
