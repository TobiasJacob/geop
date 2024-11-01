use crate::point::Point;

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
    pub fn intersects(&self, other: &BoundingBox, tolerance: f64) -> bool {
        // Check for overlap in the x dimension
        let x_overlap =
            self.min.x <= other.max.x + tolerance && self.max.x >= other.min.x - tolerance;

        // Check for overlap in the y dimension
        let y_overlap =
            self.min.y <= other.max.y + tolerance && self.max.y >= other.min.y - tolerance;

        // Check for overlap in the z dimension
        let z_overlap =
            self.min.z <= other.max.z + tolerance && self.max.z >= other.min.z - tolerance;

        // Bounding boxes intersect if there is overlap in all three dimensions
        x_overlap && y_overlap && z_overlap
    }

    pub fn max_size(&self) -> f64 {
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
