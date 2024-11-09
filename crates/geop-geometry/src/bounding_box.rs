use std::fmt::Display;

use crate::{efloat::EFloat64, point::Point};

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox { min, max }
    }

    // Checks if the 3d bounding boxes intersect in at least one point.
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        // Check for overlap in the x dimension
        let x_overlap =
            self.min.x <= other.max.x.upper_bound && self.max.x >= other.min.x.lower_bound;

        // Check for overlap in the y dimension
        let y_overlap =
            self.min.y <= other.max.y.upper_bound && self.max.y >= other.min.y.lower_bound;

        // Check for overlap in the z dimension
        let z_overlap =
            self.min.z <= other.max.z.upper_bound && self.max.z >= other.min.z.lower_bound;

        // Bounding boxes intersect if there is overlap in all three dimensions
        x_overlap && y_overlap && z_overlap
    }

    pub fn max_size(&self) -> EFloat64 {
        let diff = self.max - self.min;
        diff.x.max(diff.y).max(diff.z)
    }

    pub fn add_point(&mut self, p: Point) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }

    pub fn with_2_points(interval_self_1: Point, interval_self_2: Point) -> BoundingBox {
        let mut bounding_box = BoundingBox::new(interval_self_1, interval_self_1);
        bounding_box.add_point(interval_self_2);
        bounding_box
    }
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bounding Box:")?;
        writeln!(f, "Min: {}", self.min)?;
        writeln!(f, "Max: {}", self.max)
    }
}
