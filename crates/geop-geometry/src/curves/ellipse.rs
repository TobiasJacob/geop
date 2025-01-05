use std::fmt::Display;

use geop_algebra::efloat::EFloat64;

use crate::{
    bounding_box::BoundingBox,
    color::Category10Color,
    geometry_error::{GeometryError, GeometryResult, WithContext},
    geometry_scene::GeometryScene,
    point::Point,
    transforms::Transform,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub basis: Point,
    pub normal: Point,
    pub major_radius: Point,
    pub minor_radius: Point,
}

impl Ellipse {
    pub fn try_new(
        basis: Point,
        normal: Point,
        major_radius: Point,
        minor_radius: Point,
    ) -> GeometryResult<Ellipse> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!(
                    "Create an ellipse at {} with normal {}, major_radius {} and minor_radius {}.",
                    basis, normal, major_radius, minor_radius
                ),
                GeometryScene::with_points(vec![
                    (basis, Category10Color::Orange),
                    (basis + normal, Category10Color::Green),
                    (basis + major_radius, Category10Color::Blue),
                    (basis + minor_radius, Category10Color::Red),
                ]),
            )
        };
        if !normal.is_normalized() {
            return Err(GeometryError::new("Normal must be normalized".to_string()))
                .with_context(&error_context);
        }
        if normal.dot(major_radius) != 0.0 {
            return Err(GeometryError::new(
                "Major radius and normal must be orthogonal".to_string(),
            ))
            .with_context(&error_context);
        }
        if normal.dot(minor_radius) != 0.0 {
            return Err(GeometryError::new(
                "Minor radius and normal must be orthogonal".to_string(),
            ))
            .with_context(&error_context);
        }
        if major_radius.dot(minor_radius) != 0.0 {
            return Err(GeometryError::new(
                "Major and minor radii must be orthogonal".to_string(),
            ))
            .with_context(&error_context);
        }
        Ok(Ellipse {
            basis,
            normal,
            major_radius,
            minor_radius,
        })
    }

    fn assert_on_curve(&self, p: Point, variable_name: &str) -> GeometryResult<()> {
        if !self.on_curve(p) {
            return Err(GeometryError::new(format!(
                "Point {} {} is not on ellipse {}",
                variable_name, p, self
            )));
        }
        Ok(())
    }

    fn transform_point_to_circle(&self, p: Point) -> GeometryResult<Point> {
        self.assert_on_curve(p, "p")?;
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm_sq();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm_sq();
        Ok(Point::new(x.unwrap(), y.unwrap(), EFloat64::zero()))
    }

    fn transform_point_from_circle(&self, p: Point) -> GeometryResult<Point> {
        if p.z != 0.0 {
            return Err(GeometryError::new(format!(
                "Point p {} is has z component different from 0 in transform_point_from_circle.",
                p
            )));
        }
        if !p.is_normalized() {
            return Err(GeometryError::new(format!(
                "Point p {} is not normalized in transform_point_from_circle.",
                p
            )));
        }
        Ok(p.x * self.major_radius + p.y * self.minor_radius + self.basis)
    }

    pub fn transform(&self, transform: Transform) -> Ellipse {
        let basis = transform * self.basis;
        let normal = transform * (self.normal + self.basis) - basis;
        let major_radius = transform * (self.major_radius + self.basis) - basis;
        let minor_radius = transform * (self.minor_radius + self.basis) - basis;
        Ellipse::try_new(basis, normal, major_radius, minor_radius)
            .expect("Transform of ellipse will always succeed")
    }

    pub fn neg(&self) -> Ellipse {
        Ellipse::try_new(
            self.basis,
            -self.normal,
            self.major_radius,
            self.minor_radius,
        )
        .expect("Negation of ellipse will always succeed")
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

        let mut points = Vec::with_capacity(6);
        if let Ok(disc_x) = disc_x {
            points.push(self.basis + disc_x);
            points.push(self.basis - disc_x);
        }
        if let Ok(disc_y) = disc_y {
            points.push(self.basis + disc_y);
            points.push(self.basis - disc_y);
        }
        if let Ok(disc_z) = disc_z {
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

    fn tangent(&self, p: Point) -> GeometryResult<Point> {
        self.assert_on_curve(p, "p")?;
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm();
        let tangent = y.unwrap() * self.major_radius - x.unwrap() * self.minor_radius;
        Ok(tangent.normalize().unwrap())
    }

    fn on_curve(&self, p: Point) -> bool {
        let p = p - self.basis;
        let x = self.major_radius.dot(p) / self.major_radius.norm_sq();
        let y = self.minor_radius.dot(p) / self.minor_radius.norm_sq();
        let x = x.unwrap();
        let y = y.unwrap();
        (p.dot(self.normal) == 0.0) && (x * x + y * y == 1.0)
    }

    fn distance(&self, x: Point, y: Point) -> GeometryResult<EFloat64> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!(
                    "Calculate distance between x {} and y {} on ellipse {}",
                    x, y, self
                ),
                GeometryScene {
                    points: vec![(x, Category10Color::Orange), (y, Category10Color::Green)],
                    curves: vec![(Curve::Ellipse(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        self.assert_on_curve(x, "x").with_context(&error_context)?;
        self.assert_on_curve(y, "y").with_context(&error_context)?;
        let angle = (x - self.basis).angle(y - self.basis).unwrap();
        Ok(self.major_radius.norm() * angle)
    }

    fn interpolate(
        &self,
        start: Option<Point>,
        end: Option<Point>,
        t: f64,
    ) -> GeometryResult<Point> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!(
                    "Interpolating between {:?} and {:?} with t={}",
                    start, end, t
                ),
                GeometryScene {
                    points: vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Blue),
                    ]
                    .into_iter()
                    .filter_map(|(p, c)| p.map(|p| (p, c)))
                    .collect(),
                    curves: vec![(Curve::Ellipse(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };
        match (start, end) {
            (Some(start), Some(end)) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
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
                Ok(angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis)
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                let start = start - self.basis;
                let x_start = self.major_radius.dot(start);
                let y_start = self.minor_radius.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis)
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                let end = end - self.basis;
                let x_end = self.major_radius.dot(end);
                let y_end = self.minor_radius.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis)
            }
            (None, None) => {
                let angle = EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.major_radius + angle.sin() * self.minor_radius + self.basis)
            }
        }
    }

    fn between(&self, m: Point, start: Option<Point>, end: Option<Point>) -> GeometryResult<bool> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!("Checking if {} is between {:?} and {:?}", m, start, end),
                GeometryScene {
                    points: vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Blue),
                        (Some(m), Category10Color::Green),
                    ]
                    .into_iter()
                    .filter_map(|(p, c)| p.map(|p| (p, c)))
                    .collect(),
                    curves: vec![(Curve::Ellipse(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        self.assert_on_curve(m, "m").with_context(&error_context)?;
        match (start, end) {
            (Some(start), Some(end)) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                let start = start - self.basis;
                let end = end - self.basis;
                let m = m - self.basis;
                let angle_start = self
                    .minor_radius
                    .dot(start)
                    .atan2(self.major_radius.dot(start));
                let mut angle_end = self.minor_radius.dot(end).atan2(self.major_radius.dot(end));
                let mut angle_m = self.minor_radius.dot(m).atan2(self.major_radius.dot(m));
                if angle_end.upper_bound < angle_start.lower_bound {
                    angle_end = angle_end + EFloat64::two_pi();
                }
                if angle_m.upper_bound < angle_start.lower_bound {
                    angle_m = angle_m + EFloat64::two_pi();
                }
                Ok(angle_start <= angle_m.upper_bound && angle_m <= angle_end.lower_bound)
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                Ok(true)
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                Ok(true)
            }
            (None, None) => Ok(true),
        }
    }

    fn get_midpoint(&self, start: Option<Point>, end: Option<Point>) -> GeometryResult<Point> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!("Midpoint between {:?} and {:?}", start, end),
                GeometryScene {
                    points: vec![
                        (start, Category10Color::Orange),
                        (end, Category10Color::Blue),
                    ]
                    .into_iter()
                    .filter_map(|(p, c)| p.map(|p| (p, c)))
                    .collect(),
                    curves: vec![(Curve::Ellipse(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        match (start, end) {
            (Some(start), Some(end)) => {
                if start == end {
                    return Err(error_context(GeometryError::new(
                        "Start and end are the same".to_string(),
                    )));
                }

                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                let start_rel = self
                    .transform_point_to_circle(start)
                    .expect("Start is on curve");
                let end_rel = self
                    .transform_point_to_circle(end)
                    .expect("End is on curve");
                // println!("start_rel: {:?}", start_rel);
                // println!("end_rel: {:?}", end_rel);
                let mid = ((start_rel + end_rel) / EFloat64::two()).unwrap();
                // println!("mid: {:?}", mid);
                if mid.norm() == 0.0 {
                    return Ok(self
                        .transform_point_from_circle(
                            Point::unit_z().cross(start_rel).normalize().unwrap(),
                        )
                        .unwrap());
                }
                let mid = mid.normalize().unwrap();
                // println!("mid: {:?}", mid);
                let p1 = self
                    .transform_point_from_circle(mid)
                    .expect("Mid is on curve");
                if self.between(p1, Some(start), Some(end)).unwrap() {
                    return Ok(p1);
                } else {
                    return Ok(-mid + self.basis);
                }
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                return Ok(self.basis - (start - self.basis));
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                return Ok(self.basis - (end - self.basis));
            }
            (None, None) => {
                return Ok(self.basis + self.major_radius);
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

    fn get_bounding_box(
        &self,
        start: Option<Point>,
        end: Option<Point>,
    ) -> GeometryResult<BoundingBox> {
        if let Some(start) = start {
            assert!(self.on_curve(start));
        }
        if let Some(end) = end {
            assert!(self.on_curve(end));
        }
        match (start, end) {
            (Some(start), Some(end)) => {
                if start == end {
                    return Ok(BoundingBox::new(start, start));
                }
            }
            _ => {}
        }

        let mid_point = self.get_midpoint(start, end).unwrap();
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
            if self.between(point, start, end).unwrap() {
                bounding_box.add_point(point);
            }
        }

        Ok(bounding_box)
    }

    fn shrink_bounding_box(
        &self,
        _start: Option<Point>,
        _end: Option<Point>,
        _bounding_box: BoundingBox,
    ) -> GeometryResult<BoundingBox> {
        todo!("Implement this")
    }

    fn sort(&self, _points: Vec<Option<Point>>) -> Vec<Option<Point>> {
        todo!("Implement this")
    }
}

impl PartialEq for Ellipse {
    fn eq(&self, other: &Ellipse) -> bool {
        self.basis == other.basis
            && self.normal == other.normal
            && self.major_radius == other.major_radius
            && self.minor_radius == other.minor_radius
    }
}

impl Display for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ellipse at {} with normal {}, major radius {} and minor radius {}",
            self.basis, self.normal, self.major_radius, self.minor_radius
        )
    }
}
