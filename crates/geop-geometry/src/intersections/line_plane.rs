use crate::geometry::{surfaces::plane::Plane, curves::line3d::Line3d, points::point3d::Point3d};

pub enum LinePlaneIntersection {
    Line3d(Line3d),
    Point3d(Point3d),
    None
}

pub fn intersect(a: &Line3d, b: &Plane) -> LinePlaneIntersection {
    let n = b.normal();
    let p = b.basis;
    let v = a.direction;
    let a = a.basis;

    if (n.dot(v)).abs() < crate::EQ_THRESHOLD {
        if (n.dot(a) - n.dot(p)).abs() < crate::EQ_THRESHOLD {
            return LinePlaneIntersection::Line3d(Line3d::new(a, v));
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
        let line = Line3d::new(Point3d::new(0.0, 0.0, 0.0), Point3d::new(1.0, 0.0, 0.0));
        let plane = Plane::new(Point3d::new(0.0, 0.0, 0.0), Point3d::new(0.0, 1.0, 0.0), Point3d::new(0.0, 0.0, 1.0));
        let intersection = intersect(&line, &plane);
        match intersection {
            LinePlaneIntersection::Line3d(line) => {
                assert!((line.basis - Point3d::new(0.0, 0.0, 0.0)).norm() < crate::EQ_THRESHOLD);
                assert!((line.direction - Point3d::new(1.0, 0.0, 0.0)).norm() < crate::EQ_THRESHOLD);
            },
            _ => panic!("Expected a line-plane intersection.")
        }
    }

    #[test]
    fn test_parallel() {
        let line = Line3d::new(Point3d::new(0.0, 0.0, 0.0), Point3d::new(1.0, 0.0, 0.0));
        let plane = Plane::new(Point3d::new(0.0, 0.0, 1.0), Point3d::new(0.0, 1.0, 0.0), Point3d::new(1.0, 0.0, 0.0));
        let intersection = intersect(&line, &plane);
        match intersection {
            LinePlaneIntersection::None => (),
            _ => panic!("Expected no intersection.")
        }
    }
}