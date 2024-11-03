use crate::{
    curves::line::Line,
    point::Point,
    surfaces::{plane::Plane, SurfaceLike},
};

pub enum LinePlaneIntersection {
    Line(Line),
    Point(Point),
    None,
}

pub fn line_plane_intersection(a: &Line, b: &Plane) -> LinePlaneIntersection {
    let n = b.normal(b.basis);
    let p = b.basis;
    let v = a.direction;
    let a = a.basis;

    if (n.dot(v)).abs() <= 0.0 {
        if (n.dot(a) - n.dot(p)).abs() <= 0.0 {
            return LinePlaneIntersection::Line(Line::new(a, v).unwrap());
        } else {
            return LinePlaneIntersection::None;
        }
    }

    let t = (n.dot(p) - n.dot(a)) / n.dot(v);
    let t = t.unwrap();
    LinePlaneIntersection::Point(a + v * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let line = Line::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        )
        .unwrap();
        let plane = Plane::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
            Point::from_f64(0.0, 0.0, 1.0),
        );
        let intersection = line_plane_intersection(&line, &plane);
        match intersection {
            LinePlaneIntersection::Line(line) => {
                assert!((line.basis - Point::from_f64(0.0, 0.0, 0.0)).norm() <= 0.0);
                assert!((line.direction - Point::from_f64(1.0, 0.0, 0.0)).norm() <= 0.0);
            }
            _ => panic!("Expected a line-plane intersection."),
        }
    }

    #[test]
    fn test_parallel() {
        let line = Line::new(
            Point::from_f64(0.0, 0.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        )
        .unwrap();
        let plane = Plane::new(
            Point::from_f64(0.0, 0.0, 1.0),
            Point::from_f64(0.0, 1.0, 0.0),
            Point::from_f64(1.0, 0.0, 0.0),
        );
        let intersection = line_plane_intersection(&line, &plane);
        match intersection {
            LinePlaneIntersection::None => (),
            _ => panic!("Expected no intersection."),
        }
    }
}
