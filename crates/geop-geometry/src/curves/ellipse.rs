use crate::{
    bounding_box::BoundingBox,
    efloat::{EFloat64, SemiPositiveEFloat64},
    points::point::{NonZeroPoint, NormalizedPoint, Point},
    transforms::Transform,
    EQ_THRESHOLD,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub basis: Point,
    pub normal: NormalizedPoint,
    pub major_radius: NonZeroPoint,
    pub minor_radius: NonZeroPoint,
}

impl Ellipse {
    pub fn new(
        basis: Point,
        normal: NormalizedPoint,
        major_radius: NonZeroPoint,
        minor_radius: NonZeroPoint,
    ) -> Ellipse {
        assert!(
            normal.value.is_perpendicular(major_radius.value),
            "Major radius and normal must be orthogonal"
        );
        assert!(
            normal.value.is_perpendicular(minor_radius.value),
            "Minor radius and normal must be orthogonal"
        );
        assert!(
            major_radius.value.is_perpendicular(minor_radius.value),
            "Major and minor radii must be orthogonal"
        );
        Ellipse {
            basis,
            normal,
            major_radius,
            minor_radius,
        }
    }

    fn transform_point_to_circle(&self, p: Point) -> NormalizedPoint {
        assert!(self.on_curve(p));
        let p = p - self.basis;
        let x = self.major_radius.parallel_distance(p);
        let y = self.minor_radius.parallel_distance(p);
        Point::from_efloat(x, y, EFloat64::zero())
            .normalize()
            .unwrap()
    }

    fn transform_point_from_circle(&self, p: NormalizedPoint) -> Point {
        assert!(p.value.z.is_zero());
        p.value.x * self.major_radius.value + p.value.y * self.minor_radius.value + self.basis
    }

    pub fn transform(&self, transform: Transform) -> Ellipse {
        let basis = transform * self.basis;
        let normal = transform * (self.normal.value + self.basis) - basis;
        let major_radius = transform * (self.major_radius.value + self.basis) - basis;
        let minor_radius = transform * (self.minor_radius.value + self.basis) - basis;
        Ellipse::new(
            basis,
            normal.normalize().unwrap(),
            major_radius.expect_non_zero(),
            minor_radius.expect_non_zero(),
        )
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
        let disc_x = (self.major_radius.value.x.square() + self.minor_radius.value.x.square())
            .sqrt()
            .value;
        let disc_y = (self.major_radius.value.y.square() + self.minor_radius.value.y.square())
            .sqrt()
            .value;
        let disc_z = (self.major_radius.value.z.square() + self.minor_radius.value.z.square())
            .sqrt()
            .value;

        let disc_x = match disc_x.as_positive() {
            Some(disc_x) => Some(
                (self.major_radius.value * self.major_radius.value.x
                    + self.minor_radius.value * self.minor_radius.value.x)
                    / disc_x,
            ),
            None => None,
        };

        let disc_y = match disc_y.as_positive() {
            Some(disc_y) => Some(
                (self.major_radius.value * self.major_radius.value.y
                    + self.minor_radius.value * self.minor_radius.value.y)
                    / disc_y,
            ),
            None => None,
        };

        let disc_z = match disc_z.as_positive() {
            Some(disc_z) => Some(
                (self.major_radius.value * self.major_radius.value.z
                    + self.minor_radius.value * self.minor_radius.value.z)
                    / disc_z,
            ),
            None => None,
        };

        // vec![
        //     self.basis + disc_x,
        //     self.basis - disc_x,
        //     self.basis + disc_y,
        //     self.basis - disc_y,
        //     self.basis + disc_z,
        //     self.basis - disc_z,
        // ]
        // .iter() // Filter nan. Nan means that the ellipse is parallel to a plane, so there is no extremal point.
        // .filter(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite())
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

    fn tangent(&self, p: Point) -> NormalizedPoint {
        assert!(self.on_curve(p));
        let p = p - self.basis;
        let x = self.major_radius.parallel_distance(p);
        let y = self.minor_radius.parallel_distance(p);
        let tangent = y * self.major_radius.value - x * self.minor_radius.value;
        tangent.normalize().unwrap()
    }

    fn on_curve(&self, p: Point) -> bool {
        let p = p - self.basis;
        let x = self.major_radius.parallel_distance(p);
        let y = self.minor_radius.parallel_distance(p);
        (p.is_perpendicular(self.normal.value)) && (x.square().value + y.square().value == 1.0)
    }

    fn distance(&self, x: Point, y: Point) -> SemiPositiveEFloat64 {
        assert!(self.on_curve(x));
        assert!(self.on_curve(y));
        let angle = (x - self.basis).angle(y - self.basis).unwrap();
        let norm: SemiPositiveEFloat64 = self.major_radius.norm().into();
        norm * angle
    }

    fn interpolate(&self, start: Option<Point>, end: Option<Point>, t: f64) -> Point {
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start = start - self.basis;
                let end = end - self.basis;
                let x_start = self.major_radius.value.dot(start);
                let x_end = self.major_radius.value.dot(end);
                let y_start = self.minor_radius.value.dot(start);
                let y_end = self.minor_radius.value.dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                if angle2.upper_bound < angle1.lower_bound {
                    angle2 = angle2 + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                let angle = angle1 + EFloat64::new(t) * (angle2 - angle1);
                angle.cos() * self.major_radius.value
                    + angle.sin() * self.minor_radius.value
                    + self.basis
            }
            (Some(start), None) => {
                let start = start - self.basis;
                let x_start = self.major_radius.value.dot(start);
                let y_start = self.minor_radius.value.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::new(t * std::f64::consts::PI * 2.0);
                angle.cos() * self.major_radius.value
                    + angle.sin() * self.minor_radius.value
                    + self.basis
            }
            (None, Some(end)) => {
                let end = end - self.basis;
                let x_end = self.major_radius.value.dot(end);
                let y_end = self.minor_radius.value.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::new(t * std::f64::consts::PI * 2.0);
                angle.cos() * self.major_radius.value
                    + angle.sin() * self.minor_radius.value
                    + self.basis
            }
            (None, None) => {
                let angle = EFloat64::new(t * std::f64::consts::PI * 2.0);
                angle.cos() * self.major_radius.value
                    + angle.sin() * self.minor_radius.value
                    + self.basis
            }
        }
    }

    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> bool {
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
                    .value
                    .dot(start)
                    .atan2(self.major_radius.value.dot(start));
                let mut angle_end = self
                    .minor_radius
                    .value
                    .dot(end)
                    .atan2(self.major_radius.value.dot(end));
                let mut angle_m = self
                    .minor_radius
                    .value
                    .dot(m)
                    .atan2(self.major_radius.value.dot(m));
                if angle_end.upper_bound < angle_start.lower_bound {
                    angle_end = angle_end + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                if angle_m.upper_bound < angle_start.lower_bound {
                    angle_m = angle_m + EFloat64::new(2.0 * std::f64::consts::PI);
                }
                angle_start.upper_bound <= angle_m.lower_bound
                    && angle_m.upper_bound <= angle_end.lower_bound
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
        match (start, end) {
            (Some(start), Some(end)) => {
                assert!(self.on_curve(start));
                assert!(self.on_curve(end));
                let start_rel = self.transform_point_to_circle(start);
                let end_rel = self.transform_point_to_circle(end);
                // println!("start_rel: {:?}", start_rel);
                // println!("end_rel: {:?}", end_rel);
                let mid = (start_rel.value + end_rel.value) / EFloat64::new(2.0).expect_positive();
                // println!("mid: {:?}", mid);
                if mid.norm().value.is_zero() {
                    return self.transform_point_from_circle(
                        Point::unit_z().cross(start_rel.value).normalize().unwrap(),
                    );
                }
                let mid = mid.normalize().unwrap();
                // println!("mid: {:?}", mid);
                let p1 = self.transform_point_from_circle(mid);
                if self.between(p1, Some(start), Some(end)) {
                    return p1;
                } else {
                    return -mid.value + self.basis;
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
                return self.basis + self.major_radius.value;
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal.value * (v.dot(self.normal.value));
        v.normalize().unwrap().value * self.major_radius.norm().value + self.basis
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
            && self.major_radius.value == other.major_radius.value
            && self.minor_radius.value == other.minor_radius.value
    }
}
