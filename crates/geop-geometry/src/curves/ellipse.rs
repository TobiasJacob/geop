

use crate::{points::point::Point, transforms::Transform};

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub basis: Point,
    pub dir0: Point,
    pub dir1: Point,
}

impl Ellipse {
    pub fn new(basis: Point, dir0: Point, dir1: Point) -> Ellipse {
        Ellipse { basis, dir0, dir1 }
    }
    
    pub fn transform(&self, transform: Transform) -> Self {
        let basis = transform * self.basis;
        let dir0 = transform * (self.dir0 + self.basis) - basis;
        let dir1 = transform * (self.dir1 + self.basis) - basis;
        Ellipse::new(basis, dir0, dir1)
    }

    pub fn neg(&self) -> Ellipse {
        Ellipse::new(self.basis, -self.dir0, -self.dir1)
    }

    pub fn tangent(&self, _p: Point) -> Point {
        // let u = self.project(p).0;
        // -self.dir0 * u.sin() + self.dir1 * u.cos()
        todo!("Implement tangent")
    }
    
    pub fn on_manifold(&self, _p: Point) -> bool {
        todo!("Implement on_manifold")
    }
    
    pub fn interpolate(&self, _start: Point, _end: Point, _t: f64) -> Point {
        todo!("Implement interpolate")
    }


    // fn metric(&self, x: Point, u: TangentParameter, v: TangentParameter) -> f64 {
    //     todo!("Implement metric")
    // }

    // fn distance(&self, x: Point, y: Point) -> f64 {
    //     todo!("Implement distance")
    // }

    // fn exp(&self, x: Point, u: TangentParameter) -> Point {
    //     todo!("Implement exp")
    // }

    // fn log(&self, x: Point, y: Point) -> TangentParameter {
    //     todo!("Implement log")
    // }

    // fn parallel_transport(&self, v: TangentParameter, x: Point, y: Point) -> TangentParameter {
    //     todo!("Implement parallel_transport")
    // }

    // Checks if m is between x and y. m==x and m==y are true.
    pub fn between(&self, _m: Point, _start: Point, _end: Point) -> bool {
        todo!("Implement between")
    }

    pub fn get_midpoint(&self, _start: Point, _end: Point) -> Point {
        todo!("Implement get_midpoint")
    }
}

impl PartialEq for Ellipse {
    fn eq(&self, other: &Ellipse) -> bool {
        self.basis == other.basis && self.dir0 == other.dir0 && self.dir1 == other.dir1
    }
}
