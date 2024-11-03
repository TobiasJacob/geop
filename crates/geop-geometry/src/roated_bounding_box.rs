use crate::point::Point;

pub struct RotatedBoundingBox {
    pub center: Point,
    pub extend_1: Point,
    pub extend_2: Point,
    pub extend_3: Point,
}

impl RotatedBoundingBox {
    pub fn new(
        center: Point,
        extend_1: Point,
        extend_2: Point,
        extend_3: Point,
    ) -> RotatedBoundingBox {
        RotatedBoundingBox {
            center,
            extend_1,
            extend_2,
            extend_3,
        }
    }
}
