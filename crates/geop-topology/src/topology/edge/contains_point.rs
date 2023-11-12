use geop_geometry::{points::point::Point, EQ_THRESHOLD};

use super::{Edge, EdgeContains};

impl Edge {
    pub fn contains(&self, other: Point) -> EdgeContains {
        let u = self.project(other);
        match u {
            Some(u) => {
                if u < EQ_THRESHOLD {
                    EdgeContains::OnPoint(self.start.clone())
                } else if u > 1.0 - EQ_THRESHOLD {
                    EdgeContains::OnPoint(self.end.clone())
                } else {
                    EdgeContains::Inside
                }
            }
            None => EdgeContains::Outside,
        }
    }
}
