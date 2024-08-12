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
}
