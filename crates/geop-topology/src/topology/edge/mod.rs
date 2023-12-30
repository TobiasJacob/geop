use std::{
    fmt::{Display, Formatter},
    rc::Rc,
};

use geop_geometry::{
    curves::{curve::Curve, line::Line},
    points::point::Point,
    transforms::Transform,
};

#[derive(Clone, Debug)]
pub struct Edge {
    pub boundaries: Vec<(Rc<Point>, Rc<Point>)>,
    pub curve: Rc<Curve>,
}
// Represents an Edge, defined by a curve, and multiple intervals on the curve.
// The intervals are defined by the boundaries, and have to be ordered.
impl Edge {
    pub fn new(boundaries: Vec<(Rc<Point>, Rc<Point>)>, curve: Rc<Curve>) -> Edge {
        // assert!(boundaries.len() >= 1); // This assumption is dropped for now, an Edge can be empty
        assert!(curve.on_manifold(*boundaries[0].0));
        assert!(curve.on_manifold(*boundaries[0].1));
        for i in 0..boundaries.len() {
            // assert!(boundaries[i].0 != boundaries[i].1); // This assumption is dropped for now, an Edge can also contain points
            assert!(curve.on_manifold(*boundaries[i].0));
            assert!(curve.on_manifold(*boundaries[i].1));

            let i_prev = if i == 0 { boundaries.len() - 1 } else { i - 1 };
            assert!(curve.between(*boundaries[i_prev].1, *boundaries[i_prev].0, *boundaries[i].1));
            assert!(curve.between(*boundaries[i].0, *boundaries[i_prev].0, *boundaries[i].1));
        }
        Edge { boundaries, curve }
    }

    pub fn new_line(start: Rc<Point>, end: Rc<Point>) -> Edge {
        let l = Line::new(*start, *end - *start);
        Edge::new(vec![(start, end)], Rc::new(Curve::Line(l)))
    }

    pub fn neg(&self) -> Edge {
        let mut reversed_boundaries = self.boundaries.clone();
        reversed_boundaries.reverse();
        Edge::new(
            reversed_boundaries,
            self.curve.clone(),
        )
    }

    pub fn flip(&self) -> Edge {
        let mut reversed_boundaries = self.boundaries.clone();
        reversed_boundaries.reverse();
        Edge::new(
            reversed_boundaries,
            Rc::new(self.curve.neg()),
        )
    }

    pub fn transform(&self, transform: Transform) -> Edge {
        Edge::new(
            self.boundaries.iter().map(|(s, e)| (Rc::new(transform * **s), Rc::new(transform * **e))).collect(),
            Rc::new(self.curve.transform(transform)),
        )
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (Rc::ptr_eq(&self.curve, &other.curve) || self.curve == other.curve)
            && self.boundaries.iter().all(|(s, e)| {
                other.boundaries
                    .iter()
                    .any(|(s2, e2)| (Rc::ptr_eq(s, s2) || s == s2) && (Rc::ptr_eq(e, e2) || e == e2))
            }) || (*self.curve == other.curve.neg() && self.boundaries.iter().all(|(s, e)| {
                other.boundaries
                    .iter()
                    .any(|(s2, e2)| (Rc::ptr_eq(s, e2) || s == e2) && (Rc::ptr_eq(e, s2) || e == s2))
            }))
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self.curve.as_ref() {
            Curve::Line(_line) => write!(f, "Line"),
            Curve::Circle(_circle) => write!(f, "Circle"),
            Curve::Ellipse(_ellipse) => write!(f, "Ellipse"),
        }
    }
}
