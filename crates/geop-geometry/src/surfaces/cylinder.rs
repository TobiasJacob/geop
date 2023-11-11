use crate::{points::point::Point, curves::{curve, circle::Circle}};

use super::surface::{Surface, SurfaceCurve};

#[derive(Clone, Debug)]
pub struct Cylinder {
    pub basis: Point,
    pub extend: Point,
    pub direction: Point,
}

impl Cylinder {
    pub fn new(basis: Point, extend: Point, direction: Point) -> Cylinder {
        Cylinder {
            basis,
            extend,
            direction,
        }
    }
    
    pub fn curve_from_to(&self, p: Point, q: Point) -> Circle {
        todo!("Do the cylinder");
    }
}

impl Surface for Cylinder {
    fn normal(&self, p: Point) -> Point {
        let v = p - self.basis;
        let v = v - self.direction * v.dot(self.direction);
        let v = v.normalize();
        v
    }

    fn curve_from_to(&self, p: Point, q: Point) -> SurfaceCurve {
        todo!("Do the cylinder")
    }

    fn distance(&self, x: Point, y: Point) -> f64 {
        todo!("Do the cylinder")
    }
}

impl PartialEq for Cylinder {
    fn eq(&self, other: &Cylinder) -> bool {
        todo!("Do the cylinder but make sure that rotating the cylinder doesn't change the cylinder")
    }
}