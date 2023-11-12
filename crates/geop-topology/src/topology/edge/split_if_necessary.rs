use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::contains::edge_point::edge_contains_point;

use super::{Edge, EdgeContains};



impl Edge {
    // Checks if the edge contains the point, and if so, splits the edge into two edges.
    // It is guaranteed that this happens in order, meaning that the first edge returned will contain the start point of the original edge, and the second edge will contain the end point of the original edge.
    pub fn split_if_necessary(&self, point: Point) -> Vec<Rc<Edge>> {
        if edge_contains_point(self, point) != EdgeContains::Inside {
            return vec![Rc::new(self.clone())];
        }
        return vec![
            Rc::new(Edge::new(
                self.start.clone(),
                Rc::new(point.clone()),
                self.curve.clone(),
            )),
            Rc::new(Edge::new(
                Rc::new(point.clone()),
                self.end.clone(),
                self.curve.clone(),
            )),
        ];
    }
}