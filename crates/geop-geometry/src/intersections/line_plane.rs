use crate::geometry::{surfaces::plane::Plane, curves::line::Line, points::point::Point};

pub enum LinePlaneIntersection {
    Line3d(Line),
    Point3d(Point),
    None
}

pub fn intersect(a: &Line, b: &Plane) -> LinePlaneIntersection {
    let n = b.normal();
    let p = b.basis;
    let v = a.direction;
    let a = a.basis;

    if (n.dot(v)).abs() < crate::EQ_THRESHOLD {
        if (n.dot(a) - n.dot(p)).abs() < crate::EQ_THRESHOLD {
            return LinePlaneIntersection::Line3d(Line::new(a, v));
        } else {
            return LinePlaneIntersection::None;
        }
    }

    let t = (n.dot(p) - n.dot(a)) / n.dot(v);
    LinePlaneIntersection::Point3d(a + v * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let plane = Plane::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 1.0));
        let intersection = intersect(&line, &plane);
        match intersection {
            LinePlaneIntersection::Line3d(line) => {
                assert!((line.basis - Point::new(0.0, 0.0, 0.0)).norm() < crate::EQ_THRESHOLD);
                assert!((line.direction - Point::new(1.0, 0.0, 0.0)).norm() < crate::EQ_THRESHOLD);
            },
            _ => panic!("Expected a line-plane intersection.")
        }
    }

    #[test]
    fn test_parallel() {
        let line = Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let plane = Plane::new(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 1.0, 0.0), Point::new(1.0, 0.0, 0.0));
        let intersection = intersect(&line, &plane);
        match intersection {
            LinePlaneIntersection::None => (),
            _ => panic!("Expected no intersection.")
        }
    }
}