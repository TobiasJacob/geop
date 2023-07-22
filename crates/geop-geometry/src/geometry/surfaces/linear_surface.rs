use crate::geometry::curves::curve3d::Curve3d;
use crate::geometry::points::point3d::Point3d;

pub struct LinearSurface {
    basis: Point3d,
    u_slope: Point3d,
    v_slope: Point3d
}

impl LinearSurface {
    pub fn new(basis: Point3d, u_slope: Point3d, v_slope: Point3d) -> LinearSurface {
        LinearSurface {
            basis,
            u_slope,
            v_slope
        }
    }
}

impl Curve3d for LinearSurface {
    fn get_value(&self, u: f64) -> Point3d {
        self.basis + self.u_slope * u
    }

    fn project(&self, x: Point3d) -> f64 {
        let v = x - self.basis;
        v.dot(self.u_slope) / self.u_slope.norm()
    }

    fn normalize(&mut self) {
        self.u_slope = self.u_slope / self.u_slope.norm();
        self.v_slope = self.v_slope / self.v_slope.norm();
    }

    fn is_normalized(&self) -> bool {
        self.u_slope.norm() == 1.0 && self.v_slope.norm() == 1.0
    }
}