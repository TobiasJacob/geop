use crate::{
    curves::{circle::Circle, curve::Curve, helix::Helix, line::Line, CurveLike},
    point::Point,
    transforms::Transform,
    EQ_THRESHOLD, HORIZON_DIST,
};

use super::{
    surface::{Surface, TangentPoint},
    SurfaceLike,
};

#[derive(Clone, Debug)]
pub struct Cylinder {
    pub basis: Point,
    pub extend_dir: Point,
    pub radius: Point,
    pub normal_outwards: bool,
    dir_cross: Point,
}

impl Cylinder {
    pub fn new(basis: Point, extend_dir: Point, radius: f64, normal_outwards: bool) -> Cylinder {
        let extend_dir = extend_dir.normalize().unwrap();
        let radius = match Point::unit_x().cross(extend_dir).norm_sq()
            > Point::unit_y().cross(extend_dir).norm_sq()
        {
            true => Point::unit_x().cross(extend_dir).normalize().unwrap() * radius,
            false => Point::unit_y().cross(extend_dir).normalize().unwrap() * radius,
        };
        Cylinder {
            basis,
            extend_dir,
            radius,
            normal_outwards,
            dir_cross: extend_dir.normalize().unwrap().cross(radius),
        }
    }
    fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let normal = transform * (self.extend_dir + self.basis) - basis;
        let radius = transform * (self.radius + self.basis) - basis;
        Cylinder::new(
            basis,
            normal.normalize().unwrap(),
            radius.norm(),
            self.normal_outwards,
        )
    }

    fn neg(&self) -> Self {
        Cylinder::new(
            self.basis,
            self.extend_dir,
            self.radius.norm(),
            !self.normal_outwards,
        )
    }
}

impl SurfaceLike for Cylinder {
    fn transform(&self, transform: Transform) -> Surface {
        Surface::Cylinder(self.transform(transform))
    }

    fn normal(&self, p: Point) -> Point {
        let p = p - self.basis;
        let p = p - p.dot(self.extend_dir) * self.extend_dir;
        let normal = p.normalize().unwrap();
        if self.normal_outwards {
            normal
        } else {
            -normal
        }
    }

    fn neg(&self) -> Surface {
        Surface::Cylinder(self.neg())
    }

    fn on_surface(&self, p: Point) -> bool {
        let p_project = p - self.basis;
        let height_project = p_project.dot(self.extend_dir) * self.extend_dir;
        let radius_project = p_project - height_project;
        let dist = radius_project.norm();
        (dist - self.radius.norm()).abs() < EQ_THRESHOLD
    }

    fn metric(&self, _x: Point, u: TangentPoint, v: TangentPoint) -> f64 {
        u.dot(v)
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        assert!(self.on_surface(x));
        assert!(self.on_surface(y));
        let x = x - self.basis;
        let height_diff = (y - x).dot(self.extend_dir);
        let x = x - x.dot(self.extend_dir) * self.extend_dir;
        let y = y - self.basis;
        let y = y - y.dot(self.extend_dir) * self.extend_dir;
        let angle = x.angle(y).unwrap();
        let cylinder_dit = self.radius.norm() * angle;
        return (cylinder_dit * cylinder_dit + height_diff * height_diff).sqrt();
    }

    fn exp(&self, x: Point, u: TangentPoint) -> Point {
        assert!(self.on_surface(x));
        let x = x - self.basis;
        let height_diff = u.dot(self.extend_dir);
        let u = u - height_diff * self.extend_dir;
        let u_norm = u.norm();
        if u_norm < EQ_THRESHOLD {
            return x + height_diff * self.extend_dir;
        }
        let u_normalized = u / u_norm;
        self.basis
            + self.extend_dir * height_diff
            + x * u_norm.cos()
            + u_normalized * self.radius.norm() * u_norm.sin()
    }

    fn log(&self, x: Point, y: Point) -> Option<TangentPoint> {
        assert!(self.on_surface(x), "{:?} {:?}", x, y);
        assert!(self.on_surface(y), "{:?} {:?}", x, y);
        let x = x - self.basis;
        let y = y - self.basis;
        let height_diff = y.dot(self.extend_dir) - x.dot(self.extend_dir);
        let x = x - x.dot(self.extend_dir) * self.extend_dir;
        let y = y - y.dot(self.extend_dir) * self.extend_dir;
        let angle = x.angle(y).unwrap();
        if angle < EQ_THRESHOLD {
            return Some(height_diff * self.extend_dir);
        }
        let x = x.normalize().unwrap();
        let y = y.normalize().unwrap();

        let dir = y - x.dot(y) * x;
        assert!(dir.dot(self.extend_dir).abs() < EQ_THRESHOLD);

        // This means that we are on the opposite side of the cylinder
        if dir.norm() < EQ_THRESHOLD {
            return None;
        }

        Some(height_diff * self.extend_dir + dir / dir.norm() * angle)
    }

    fn parallel_transport(
        &self,
        _v: Option<TangentPoint>,
        _x: Point,
        _y: Point,
    ) -> Option<TangentPoint> {
        todo!()
    }

    fn geodesic(&self, p: Point, q: Point) -> Curve {
        assert!(self.on_surface(p));
        assert!(self.on_surface(q));
        assert!(p != q);
        let p_loc = p - self.basis;
        let q_loc = q - self.basis;
        let p_height = p_loc.dot(self.extend_dir);
        let q_height = q_loc.dot(self.extend_dir);
        let p_proj = p_loc - p_height * self.extend_dir;
        let q_proj = q_loc - self.extend_dir * q_height;
        let angle = p_proj.angle(q_proj).unwrap();
        if angle < EQ_THRESHOLD {
            return Curve::Line(Line::new(p, q - p));
        }
        let helix_basis = self.basis + p_height * self.extend_dir;
        let helix_radius = p_proj;
        let helix_pitch =
            self.extend_dir * (q_height - p_height) * 2.0 * std::f64::consts::PI / angle;
        if helix_pitch.norm() < EQ_THRESHOLD {
            return Curve::Circle(Circle::new(
                self.basis + p_height * self.extend_dir,
                self.extend_dir.normalize().unwrap(),
                helix_radius.norm(),
            ));
        }
        let helix = Curve::Helix(Helix::new(helix_basis, helix_pitch, helix_radius, true));
        assert!(helix.on_curve(p));
        if !helix.on_curve(q) {
            let helix = Helix::new(helix_basis, helix_pitch, helix_radius, false);
            assert!(helix.on_curve(q));
            return Curve::Helix(helix);
        }
        helix
    }

    fn point_grid(&self, density: f64) -> Vec<Point> {
        let n = (16.0 * density) as usize;
        let m = (16.0 * density) as usize;
        let mut points = Vec::with_capacity(n * m);
        for i in 0..n {
            for j in 0..m {
                let theta = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                let v = j as f64 / (m as f64 - 1.0);
                let point = self.basis
                    + (v - 0.5) * HORIZON_DIST * self.extend_dir
                    + theta.cos() * self.radius
                    + theta.sin() * self.dir_cross;
                assert!(self.on_surface(point));
                points.push(point);
            }
        }
        points
    }

    fn project(&self, point: Point) -> Point {
        let point = point - self.basis;
        let height_diff = point.dot(self.extend_dir) * self.extend_dir;
        let point = point - height_diff;
        point.normalize().unwrap() * self.radius.norm() + height_diff + self.basis
    }

    fn unsigned_l2_squared_distance_gradient(&self, point: Point) -> Option<Point> {
        let point = point - self.basis;
        let point = point - point.dot(self.extend_dir) * self.extend_dir;
        let dist = point.norm();
        if dist < EQ_THRESHOLD {
            return None;
        }
        let normal = point / dist;
        let grad = -normal * dist;
        Some(grad)
    }
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Cylinder) -> bool {
        self.basis == other.basis
            && (self.radius.norm() - other.radius.norm() < EQ_THRESHOLD)
            && self.extend_dir.is_parallel(other.extend_dir)
            && self.normal_outwards == other.normal_outwards
    }
}
