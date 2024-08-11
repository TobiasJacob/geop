use crate::points::point::Point;

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
        (self.min.x <= other.max.x || self.max.x >= other.min.x)
            && (self.min.y <= other.max.y || self.max.y >= other.min.y)
            && (self.min.z <= other.max.z || self.max.z >= other.min.z)
    }
}
