use crate::{
    curve_surface_intersection::line_plane::{line_plane_intersection, LinePlaneIntersection},
    curves::line::Line,
    surfaces::{plane::Plane, SurfaceLike},
};

pub enum PlanePlaneIntersection {
    Plane(Plane),
    Line(Line),
    None,
}

pub fn plane_plane_intersection(a: &Plane, b: &Plane) -> PlanePlaneIntersection {
    let n_a = a.normal(a.basis);
    let n_b = b.normal(b.basis);
    let b_b = b.basis;

    if a.is_parallel(b) {
        if a.on_surface(b_b) {
            PlanePlaneIntersection::Plane(a.clone())
        } else {
            PlanePlaneIntersection::None
        }
    } else {
        let v = n_a.cross(n_b).normalize().unwrap();
        let c = Line::new(b_b, v.cross(n_b));

        match line_plane_intersection(&c, &a) {
            LinePlaneIntersection::Point(p) => {
                return PlanePlaneIntersection::Line(Line::new(p, v));
            }
            _ => panic!("Line plane intersection should return a point!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    use super::*;

    #[test]
    fn test_plane_plane_intersection_planes() {
        // Simplest case where the planes intersect in a line
        let top = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        );

        let askew = Plane::new(
            Point::new(1.0, 1.0, 1.0),
            Point::new(1.0, 0.0, -0.2),
            Point::new(0.0, 1.0, 0.0),
        );

        match plane_plane_intersection(&top, &askew) {
            PlanePlaneIntersection::Line(line) => {
                assert_eq!(line.direction, Point::new(0.0, 1.0, 0.0));
                assert!(top.on_surface(line.basis));
                assert!(askew.on_surface(line.basis));
            }
            _ => panic!("Intersection should be a line"),
        }
    }

    #[test]
    fn test_plane_plane_intersection_lines_degenerate() {
        // This tests the case where the planes intersect in a line
        // And the line passes through the basis of one of the planes
        let plane1 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let plane2 = Plane::new(
            Point::new(0.0, 0.0, -1.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );

        match plane_plane_intersection(&plane1, &plane2) {
            PlanePlaneIntersection::Line(line) => {
                assert_eq!(line.direction, Point::new(0.0, -1.0, 0.0));
                assert!(plane1.on_surface(line.basis));
                assert!(plane2.on_surface(line.basis));
            }
            _ => panic!("Intersection should be a line"),
        }

        match plane_plane_intersection(&plane2, &plane1) {
            PlanePlaneIntersection::Line(line) => {
                // The direction of this line intersection is the opposite of the one above
                assert_eq!(line.direction, Point::new(0.0, 1.0, 0.0));
                assert!(plane1.on_surface(line.basis));
                assert!(plane2.on_surface(line.basis));
            }
            _ => panic!("Intersection should be a line"),
        }
    }
}
