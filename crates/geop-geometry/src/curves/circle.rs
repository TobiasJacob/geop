use std::fmt::Display;

use geop_algebra::efloat::EFloat64;

use crate::{
    bounding_box::BoundingBox,
    color::Category10Color,
    geometry_error::{ElevateToGeometry, GeometryError, GeometryResult, WithContext},
    geometry_scene::GeometryScene,
    point::Point,
    transforms::Transform,
};

use super::{curve::Curve, CurveLike};

#[derive(Debug, Clone)]
pub struct Circle {
    pub basis: Point,
    pub normal: Point,
    pub radius: Point,
    dir_cross: Point,
}

pub enum CircleTransform {
    Circle(Circle),
    Ellipse(), // TODO: Implement this
}

impl Circle {
    pub fn try_new(basis: Point, normal: Point, radius: EFloat64) -> GeometryResult<Circle> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!(
                    "Create a circle at {} with normal {} and radius {}.",
                    basis, normal, radius
                ),
                GeometryScene::with_points(vec![
                    (basis, Category10Color::Orange),
                    (basis + normal, Category10Color::Green),
                    (basis + radius, Category10Color::Blue),
                ]),
            )
        };
        if !normal.is_normalized() {
            return Err(GeometryError::new("Normal must be normalized".to_string()))
                .with_context(&error_context);
        }
        if radius <= 0.0 {
            return Err(GeometryError::new("Radius must be positive".to_string()))
                .with_context(&error_context);
        }
        let radius = match Point::unit_x().cross(normal).norm_sq().lower_bound
            > Point::unit_y().cross(normal).norm_sq().lower_bound
        {
            true => Point::unit_x().cross(normal).normalize().unwrap() * radius,
            false => Point::unit_y().cross(normal).normalize().unwrap() * radius,
        };
        Ok(Circle {
            basis,
            normal,
            radius,
            dir_cross: normal.cross(radius),
        })
    }

    fn assert_on_curve(&self, p: Point, variable_name: &str) -> GeometryResult<()> {
        if !self.on_curve(p) {
            return Err(GeometryError::new(format!(
                "Point {} {} is not on circle {}",
                variable_name, p, self
            )));
        }
        Ok(())
    }

    pub fn transform(&self, transform: Transform) -> CircleTransform {
        let basis = transform * self.basis;
        let normal = transform * (self.normal + self.basis) - basis;
        let radius = transform * (self.radius + self.basis) - basis;
        assert!(transform.uniform_scale_factor() > 0.0, "Circle can only be transformed with uniform scaling. An extension of this method is planned to return ellipsis.");
        CircleTransform::Circle(
            Circle::try_new(basis, normal.normalize().unwrap(), radius.norm())
                .expect("Circle should still be a circle after transform"),
        )
    }

    pub fn neg(&self) -> Circle {
        Circle::try_new(self.basis, -self.normal, self.radius.norm())
            .expect("Circle parameters should be valid")
    }
}

impl CurveLike for Circle {
    fn transform(&self, transform: Transform) -> Curve {
        match self.transform(transform) {
            CircleTransform::Circle(circle) => Curve::Circle(circle),
            CircleTransform::Ellipse() => todo!("Implement this"),
        }
    }

    fn neg(&self) -> Curve {
        Curve::Circle(self.neg())
    }

    fn tangent(&self, p: Point) -> GeometryResult<Point> {
        self.assert_on_curve(p, "p")?;
        Ok(self.normal.cross(p - self.basis).normalize().unwrap())
    }

    fn on_curve(&self, p: Point) -> bool {
        (p - self.basis).dot(self.normal) == 0.0
            && ((p - self.basis).norm() - self.radius.norm()) == 0.0
    }

    fn distance(&self, x: Point, y: Point) -> GeometryResult<EFloat64> {
        let error_context = |err: GeometryError| {
            err.with_context_scene(
                format!("Calculate the distance between {} and {}.", x, y),
                GeometryScene {
                    points: vec![(x, Category10Color::Orange), (y, Category10Color::Orange)],
                    curves: vec![(Curve::Circle(self.clone()), Category10Color::Gray)],
                    surfaces: vec![],
                },
            )
        };

        self.assert_on_curve(x, "x").with_context(&error_context)?;
        self.assert_on_curve(y, "y").with_context(&error_context)?;
        let angle = (x - self.basis)
            .angle(y - self.basis)
            .elevate(&error_context)?;
        Ok(self.radius.norm() * angle)
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
                    curves: vec![(Curve::Circle(self.clone()), Category10Color::Gray)],
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
                let x_start = self.radius.dot(start);
                let x_end = self.radius.dot(end);
                let y_start = self.dir_cross.dot(start);
                let y_end = self.dir_cross.dot(end);
                let angle1 = y_start.atan2(x_start);
                let mut angle2 = y_end.atan2(x_end);
                if angle1 == angle2 {
                    return Err(error_context(GeometryError::new(
                        "The two angles are equal".to_string(),
                    )));
                }
                if angle2.upper_bound < angle1.lower_bound {
                    angle2 = angle2 + EFloat64::two_pi();
                }
                let angle = angle1 + EFloat64::from(t) * (angle2 - angle1);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
            (Some(start), None) => {
                self.assert_on_curve(start, "start")
                    .with_context(&error_context)?;
                let start = start - self.basis;
                let x_start = self.radius.dot(start);
                let y_start = self.dir_cross.dot(start);
                let angle1 = y_start.atan2(x_start);
                let angle = angle1 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
            (None, Some(end)) => {
                self.assert_on_curve(end, "end")
                    .with_context(&error_context)?;
                let end = end - self.basis;
                let x_end = self.radius.dot(end);
                let y_end = self.dir_cross.dot(end);
                let angle2 = y_end.atan2(x_end);
                let angle = angle2 + EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
            (None, None) => {
                let angle = EFloat64::from(t * std::f64::consts::PI * 2.0);
                Ok(angle.cos() * self.radius + angle.sin() * self.dir_cross + self.basis)
            }
        }
    }

    // TODO: Assert start != end
    // Checks if m is between x and y. m==x and m==y are true.
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
                    curves: vec![(Curve::Circle(self.clone()), Category10Color::Gray)],
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
                let angle_start = self.dir_cross.dot(start).atan2(self.radius.dot(start));
                let mut angle_end = self.dir_cross.dot(end).atan2(self.radius.dot(end));
                let mut angle_m = self.dir_cross.dot(m).atan2(self.radius.dot(m));
                if angle_end.upper_bound < angle_start.lower_bound {
                    angle_end = angle_end + EFloat64::two_pi();
                }
                if angle_m.upper_bound < angle_start.lower_bound {
                    angle_m = angle_m + EFloat64::two_pi();
                }
                Ok(angle_start <= angle_m.upper_bound && angle_m <= angle_end.upper_bound)
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
                    curves: vec![(Curve::Circle(self.clone()), Category10Color::Gray)],
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
                let start_rel = start - self.basis;
                let end_rel = end - self.basis;
                let mid = ((start_rel + end_rel) / EFloat64::two()).unwrap();
                if mid.norm() == 0.0 {
                    return Ok(self.normal.cross(start_rel).normalize().unwrap()
                        * self.radius.norm()
                        + self.basis);
                }
                let mid = mid.normalize().unwrap() * self.radius.norm();
                let p1 = mid + self.basis;
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
                return Ok(self.basis + self.radius);
            }
        }
    }

    fn project(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.normal * (v.dot(self.normal));
        v.normalize().unwrap() * self.radius.norm() + self.basis
    }

    fn get_bounding_box(
        &self,
        _interval_self: Option<Point>,
        _midpoint_self: Option<Point>,
    ) -> GeometryResult<BoundingBox> {
        todo!("Implement this")
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

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        self.basis == other.basis && self.normal == other.normal && self.radius == other.radius
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Circle at {} with normal {} and radius {}",
            self.basis, self.normal, self.radius
        )
    }
}
