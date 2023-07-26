use crate::{geometry::points::point::Point, EQ_THRESHOLD};

use super::curve::Curve;

pub struct Circle {
    pub basis: Point,
    pub normal: Point,
    pub radius: f64,
    pub is_normalized: bool
}

impl Circle {
    pub fn new(basis: Point, normal: Point, radius: f64) -> Circle {
        Circle {
            basis,
            normal,
            radius,
            is_normalized: false
        }
    }

    fn project(&self, x: &Point) -> f64 {
        let v = *x - self.basis;
        let v = v - self.normal * v.dot(self.normal);
        let angle = v.y.atan2(v.x);
        angle
    }
}

impl Curve for Circle {
    fn point_at(&self, u: f64) -> Point {
        let x = self.basis.x + self.radius * u.cos();
        let y = self.basis.y + self.radius * u.sin();
        let z = self.basis.z;
        Point::new(x, y, z)
    }

    fn interval(&self, start: &Point, end: &Point) -> (f64, f64) {
        let start_angle = self.project(start);
        let end_angle = self.project(end);
        if start_angle + EQ_THRESHOLD <= end_angle {
            (start_angle, end_angle)
        } else {
            (start_angle, end_angle + 2.0 * std::f64::consts::PI)
        }
    }

    fn normalize(&mut self) {
        if !self.is_normalized {
            self.normal = self.normal / self.normal.norm();
            self.is_normalized = true;
        }
    }

    fn is_normalized(&self) -> bool {
        self.is_normalized
    }

    fn period(&self) -> f64 {
        2.0 * std::f64::consts::PI
    }
}

#[cfg(test)]
mod tests {
    use crate::EQ_THRESHOLD;

    use super::*;

    #[test]
    fn test_circle3d() {
        let circle = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        assert!((circle.point_at(0.0) - Point::new(1.0, 0.0, 0.0)).norm() < EQ_THRESHOLD);
        assert!((circle.point_at(std::f64::consts::PI / 2.0) - Point::new(0.0, 1.0, 0.0)).norm() < EQ_THRESHOLD);
        assert!((circle.point_at(std::f64::consts::PI) - Point::new(-1.0, 0.0, 0.0)).norm() < EQ_THRESHOLD);
        assert!((circle.point_at(3.0 * std::f64::consts::PI / 2.0) - Point::new(0.0, -1.0, 0.0)).norm() < EQ_THRESHOLD);
        assert!((circle.point_at(2.0 * std::f64::consts::PI) - Point::new(1.0, 0.0, 0.0)).norm() < EQ_THRESHOLD);

        let circle = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 2.0);
        assert!((circle.point_at(0.0) - Point::new(2.0, 0.0, 0.0)).norm() < EQ_THRESHOLD);
    }

    #[test]
    fn test_inteval() {
        let circle = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        let (start, end) = circle.interval(&Point::new(1.0, 0.0, 0.0), &Point::new(0.0, 1.0, 0.0));
        assert!((start - 0.0).abs() < EQ_THRESHOLD);
        assert!((end - std::f64::consts::PI / 2.0).abs() < EQ_THRESHOLD);
    }

    #[test]
    fn test_interval_at_boundary() {
        let circle = Circle::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 1.0), 1.0);
        let (start, end) = circle.interval(&Point::new(-1.0, 0.0, 0.0), &Point::new(-1.0, 0.0, 0.0));
        assert!((start - end + std::f64::consts::PI * 2.0).abs() < EQ_THRESHOLD);
    }
}