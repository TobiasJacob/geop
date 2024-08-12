use crate::{points::point::Point, EQ_THRESHOLD};

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
        (self.min.x <= other.max.x + EQ_THRESHOLD || self.max.x + EQ_THRESHOLD >= other.min.x)
            && (self.min.y <= other.max.y + EQ_THRESHOLD
                || self.max.y + EQ_THRESHOLD >= other.min.y)
            && (self.min.z <= other.max.z + EQ_THRESHOLD
                || self.max.z + EQ_THRESHOLD >= other.min.z)
    }

    pub fn min_size(&self) -> f64 {
        let diff = self.max - self.min;
        diff.x.min(diff.y).min(diff.z)
    }

    pub fn add_point(&mut self, p: Point) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }
}
