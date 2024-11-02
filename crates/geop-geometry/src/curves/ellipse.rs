use crate::{bounding_box::BoundingBox, efloat::EFloat64, point::Point, transforms::Transform};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub basis: Point,
    pub normal: Point,
    pub major_radius: Point,
    pub minor_radius: Point,
}

impl Ellipse {
    pub fn new(basis: Point, normal: Point, major_radius: Point, minor_radius: Point) -> Ellipse {
        assert!(normal.is_normalized());
        assert!(
            normal.dot(major_radius) == 0.0,
            "Major radius and normal must be orthogonal"
        );
        assert!(
            normal.dot(minor_radius) == 0.0,
            "Minor radius and normal must be orthogonal"
        );
        assert!(
            major_radius.dot(minor_radius) == 0.0,
            "Major and minor radii must be orthogonal"
        );
        Ellipse {
            basis,
            normal,
            major_radius,
            minor_radius,
        }
    }

    fn transform_point_to_circle(&self, p: Point) -> Point {
        assert!(self.on_curve(p));
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm_sq();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm_sq();
        Point::new(x.unwrap(), y.unwrap(), EFloat64::zero())
    }

    fn transform_point_from_circle(&self, p: Point) -> Point {
        assert!(p.z == 0.0);
        assert!(p.is_normalized());
        p.x * self.major_radius + p.y * self.minor_radius + self.basis
    }

    pub fn transform(&self, transform: Transform) -> Ellipse {
        let basis = transform * self.basis;
        let normal = transform * (self.normal + self.basis) - basis;
        let major_radius = transform * (self.major_radius + self.basis) - basis;
        let minor_radius = transform * (self.minor_radius + self.basis) - basis;
        Ellipse::new(basis, normal, major_radius, minor_radius)
    }

    pub fn neg(&self) -> Ellipse {
        Ellipse::new(
            self.basis,
            -self.normal,
            self.major_radius,
            self.minor_radius,
        )
    }

    pub fn get_extremal_points(&self) -> Vec<Point> {
        let disc_x = (self.major_radius.x * self.major_radius.x
            + self.minor_radius.x * self.minor_radius.x)
            .sqrt()
            .unwrap();
        let disc_x = (self.major_radius * self.major_radius.x
            + self.minor_radius * self.minor_radius.x)
            / disc_x;
        let disc_y = (self.major_radius.y * self.major_radius.y
            + self.minor_radius.y * self.minor_radius.y)
            .sqrt()
            .unwrap();
        let disc_y = (self.major_radius * self.major_radius.y
            + self.minor_radius * self.minor_radius.y)
            / disc_y;
        let disc_z = (self.major_radius.z * self.major_radius.z
            + self.minor_radius.z * self.minor_radius.z)
            .sqrt()
            .unwrap();
        let disc_z = (self.major_radius * self.major_radius.z
            + self.minor_radius * self.minor_radius.z)
            / disc_z;

        // vec![
        //     self.basis + disc_x,
        //     self.basis - disc_x,
        //     self.basis + disc_y,
        //     self.basis - disc_y,
        //     self.basis + disc_z,
        //     self.basis - disc_z,
        // ]
        // .iter() // Filter nan. Nan means that the ellipse is parallel to a plane, so there is no extremal point.
        // .cloned()
        // .collect()
        let mut points = Vec::with_capacity(6);
        if let Some(disc_x) = disc_x {
            points.push(self.basis + disc_x);
            points.push(self.basis - disc_x);
        }
        if let Some(disc_y) = disc_y {
            points.push(self.basis + disc_y);
            points.push(self.basis - disc_y);
        }
        if let Some(disc_z) = disc_z {
            points.push(self.basis + disc_z);
            points.push(self.basis - disc_z);
        }
        points
    }
}

impl CurveLike for Ellipse {
    fn transform(&self, transform: Transform) -> Curve {
        Curve::Ellipse(self.transform(transform))
    }

    fn neg(&self) -> Curve {
        Curve::Ellipse(self.neg())
    }

    fn tangent(&self, p: Point) -> Point {
        assert!(self.on_curve(p));
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm();
        let tangent = y.unwrap() * self.major_radius - x.unwrap() * self.minor_radius;
        tangent.normalize().unwrap()
    }

    fn on_curve(&self, p: Point) -> bool {
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm_sq();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm_sq();
        let x = x.unwrap();
        let y = y.unwrap();
        (p.dot(self.normal) == 0.0) && (x * x + y * y == 1.0)
    }

    fn distance(&self, x: Point, y: Point) -> EFloat64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis).unwrap();
        self.major_radius.norm() * angle
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let x_start = self.major_radius.dot(start);
                let x_end = self.major_radius.dot(end);
                let y_start = self.minor_radius.dot(start);
                let y_end = self.minor_radius.dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                assert!(angle2 != angle1);
                if angle2.upper_bound < angle1.lower_bound {
                    angle2 = EFloat64::two_pi();
                }
                let angle = angle1 + EFloat64::from(t) * (angle2 - angle1);
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
            (Some(start), None) => {
                let start = start - self.basis;
                let x_start = self.major_radius.dot(start);
                let y_start = self.minor_radius.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
            (None, Some(end)) => {
                let end = end - self.basis;
                let x_end = self.major_radius.dot(end);
                let y_end = self.minor_radius.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
            (None, None) => {
                let angle = EFloat64::from(t * std::f64::consts::PI * 2.0);
                angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis
            }
        }
    }

    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
        assert!(start != end);
        assert!(self.on_curve(m));
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let m = m - self.basis;
                let angle_start = self
                    .minor_radius
                    .dot(start)
                    .atan2(self.major_radius.dot(start));
                let mut angle_end = self.minor_radius.dot(end).atan2(self.major_radius.dot(end));
                let mut angle_m = self.minor_radius.dot(m).atan2(self.major_radius.dot(m));
                assert!(angle_end != angle_start);
                if angle_end.upper_bound < angle_start.lower_bound {
                    angle_end = angle_end + EFloat64::two_pi();
                }
                if angle_m.upper_bound < angle_start.lower_bound {
                    angle_m = angle_m + EFloat64::two_pi();
                }
                angle_start <= angle_m.upper_bound && angle_m <= angle_end.lower_bound
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                true
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                true
            }
            (None, None) => true,
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> Point {
        assert!(start != end);
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start_rel = self.transform_point_to_circle(start);
                let end_rel = self.transform_point_to_circle(end);
                // println!("start_rel: {:?}", start_rel);
                // println!("end_rel: {:?}", end_rel);
                let mid = ((start_rel + end_rel) / EFloat64::two()).unwrap();
                // println!("mid: {:?}", mid);
                if mid.norm() == 0.0 {
                    return self.transform_point_from_circle(
                        Point::unit_z().cross(start_rel).normalize().unwrap(),
                    );
                }
                let mid = mid.normalize().unwrap();
                // println!("mid: {:?}", mid);
                let p1 = self.transform_point_from_circle(mid);
                if self.between(p1, Some(start), Some(end)) {
                    return p1;
                } else {
                    return -mid + self.basis;
                }
            }
            (Some(start), None) => {
                assert!(self.on_curve(start));
                return self.basis - (start - self.basis);
            }
            (None, Some(end)) => {
                assert!(self.on_curve(end));
                return self.basis - (end - self.basis);
            }
            (None, None) => {
                return self.basis + self.major_radius;
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal * (v.dot(self.normal));
        let x = self.major_radius.dot(v) / self.major_radius.norm_sq();
        let y = self.minor_radius.dot(v) / self.minor_radius.norm_sq();
        let x = x.unwrap();
        let y = y.unwrap();
        let norm = (x * x + y * y).sqrt().unwrap();
        if norm == 0.0 {
            return self.basis + self.major_radius;
        }
        let x = x / norm;
        let y = y / norm;
        let x = x.unwrap();
        let y = y.unwrap();
        self.basis + x * self.major_radius + y * self.minor_radius
    }

    fn get_bounding_box(&self, start: Option<Point>, end: Option<Point>) -> BoundingBox {
        if let Some(start) = start {
            assert!(self.on_curve(start));
        }
        if let Some(end) = end {
            assert!(self.on_curve(end));
        }
        match (start, end) {
            (Some(start), Some(end)) => {
                if start == end {
                    return BoundingBox::new(start, start);
                }
            }
            _ => {}
        }

        let mid_point = self.get_midpoint(start, end);
        let mut bounding_box = BoundingBox::new(mid_point, mid_point);
        if let Some(start) = start {
            bounding_box.add_point(start);
        }
        if let Some(end) = end {
            bounding_box.add_point(end);
        }
        // Now find the max x, y, z values
        // https://iquilezles.org/articles/ellipses/

        let extremal_points = self.get_extremal_points();
        for point in extremal_points {
            if self.between(point, start, end) {
                bounding_box.add_point(point);
            }
        }

        bounding_box
    }
    fn sort(&self, _points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        todo!("Implement this")
    }
}

// Implement partial equality for ellipse
impl PartialEq for Ellipse {
    fn eq(&self, other: &Ellipse) -> bool {
        self.basis == other.basis
            && self.normal == other.normal
            && self.major_radius == other.major_radius
            && self.minor_radius == other.minor_radius
    }
}
