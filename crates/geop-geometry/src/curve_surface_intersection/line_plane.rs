use crate::{curves::line::Line, points::point::Point, surfaces::plane::Plane};

pub enum LinePlaneIntersection {
    Line(Line),
    Point(Point),
    None,
}

pub fn line_plane_intersection(a: &Line, b: &Plane) -> LinePlaneIntersection {
    let n = b.normal();
    let p = b.basis;
    let v = a.direction;
    let a = a.basis;

    if (n.dot(v)).abs() < crate::EQ_THRESHOLD {
        if (n.dot(a) - n.dot(p)).abs() < crate::EQ_THRESHOLD {
            return LinePlaneIntersection::Line(Line::new(a, v));
        } else {
            return LinePlaneIntersection::None;
        }
    }

    let t = (n.dot(p) - n.dot(a)) / n.dot(v);
    LinePlaneIntersection::Point(a + v * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let plane = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );
        let intersection = line_plane_intersection(&line, &plane);
        match intersection {
            LinePlaneIntersection::Line(line) => {
                assert!((line.basis - Point::new(0.0, 0.0, 0.0)).norm() < crate::EQ_THRESHOLD);
                assert!((line.direction - Point::new(1.0, 0.0, 0.0)).norm() < crate::EQ_THRESHOLD);
            }
            _ => panic!("Expected a line-plane intersection."),
        }
    }

    #[test]
    fn test_parallel() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let plane = Plane::new(
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );
        let intersection = line_plane_intersection(&line, &plane);
        match intersection {
            LinePlaneIntersection::None => (),
            _ => panic!("Expected no intersection."),
        }
    }
}
