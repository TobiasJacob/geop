use std::rc::Rc;

use geop_geometry::{geometry::{points::point::Point, curves::curve::CurveParameterSpace}, intersections::curve_curve::IntersectableCurve3d, EQ_THRESHOLD};

use crate::topology::Vertex::Vertex;

pub enum LinearEdgeCurve {
    Line(Line),
    Circle(Circle),
    Ellipse(Ellipse),
}

pub struct LinearEdge {
    pub start: Point,
    pub end: Point,
    pub curve: Rc<LinearEdgeCurve>
}

pub enum CircularEdgeCurve {
    Circle(Circle),
    Ellipse(Ellipse),
}
pub struct CircularEdge {
    pub origin: Point,
    pub curve: Rc<CircularEdgeCurve>
}

// TODO: Implement an periodic / circular edge
impl Edge {
    pub fn new(vertices: [Vertex; 2], curve: Rc<IntersectableCurve3d>) -> Edge {
        Edge {
            vertices,
            curve,
            parameter_space: curve.curve().construct_parameter_space(vertices[0], vertices[1])
        }
    }

    pub fn interval(&self) -> (f64, f64) {
        return self.curve.curve().interval(&self.vertices[0].point, &self.vertices[1].point);
    }

    pub fn length(&self) -> f64 {
        self.parameter_space.length()
    }

    pub fn point_at(&self, u: f64) -> Point {
        let (start, end) = self.interval();
        self.curve.curve().point_at(start + u)
    }

    pub fn project(&self, point: &Point) -> f64 {
        let (start, end) = self.interval();
        let (start, u) = self.curve.curve().interval(&self.vertices[0].point, point);
        assert!(u <= end);
        return u - start;
    }

    pub fn rasterize(&self) -> Vec<Point> {
        let num_points = 40 as usize;
        let (start, end) = self.interval();

        (0..num_points).map(|i| {
            let t = i as f64 / (num_points - 1) as f64;
            let point = self.curve.curve().point_at(t);
            let point = point + (end - start) * t;
            point
        }).collect()
    }


    // Returns a sorted list of intersections. The intersections are sorted by the parameter of the first curve. Start and end points are not included.
    pub fn inner_intersections(&self, other: &Edge) -> Vec<Point> {
        let intersections = self.curve.intersections(&other.curve);
        let (u_min, u_max) = self.interval();
        match intersections {
            geop_geometry::intersections::curve_curve::IntersectableCurve3dResult::MultiPoint(points) => {
                points.into_iter().filter(|p| {
                    let (_, u) = self.curve.curve().interval(&self.vertices[0].point, &p);
                    u_min + EQ_THRESHOLD < u && u < u_max - EQ_THRESHOLD
                }).collect::<Vec<Point>>()
            },
            geop_geometry::intersections::curve_curve::IntersectableCurve3dResult::Point3d(point) => {
                let (_, u) = self.curve.curve().interval(&self.vertices[0].point, &point);
                if u_min + EQ_THRESHOLD < u && u < u_max - EQ_THRESHOLD {
                    vec![point]
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new()
        }
    }

    // Splits this curve into subcurves at the intersections with the other curve.
    // Returns a sorted List of new edges.
    // This is an important operation, as it makes sure that the edges are not intersecting anymore except at the end points.
    // Especially, calling this function twice will not return any new edges.
    // Also, calling intersections with any 2 edges will not return any intersections besides the end points.
    pub fn split(&self, other: &Edge) -> (Vec<Edge>, Vec<Edge>) {
        let intersections_self = self.inner_intersections(other);
        if intersections_self.len() == 0 {
            return (vec![*self], vec![*other]);
        }

        let vertices_self = intersections_self.into_iter().map(|p| {
            Vertex { point: Rc::new(p) }
        }).collect::<Vec<Vertex>>();
        // Creates a shallow copy of vertices_a, meaning that they still reference the same points
        let mut vertices_other = vertices_self.clone();
        vertices_other.sort_by(|b1, b2| other.curve.curve().interval(&self.vertices[0].point, &b1.point).1.total_cmp(&other.curve.curve().interval(&self.vertices[0].point, &b2.point).1));

        let mut edges_self = Vec::with_capacity(vertices_self.len() + 1);
        let mut edges_other = Vec::with_capacity(vertices_self.len() + 1);

        // Only inner intersections are relevant, as the end points are already connected.
        edges_self.push(Edge::new([self.vertices[0].clone(), vertices_self[0].clone()], self.curve.clone()));
        for i in 0..vertices_self.len() - 1 {
            edges_self.push(Edge::new([vertices_self[i].clone(), vertices_self[i + 1].clone()], self.curve.clone()));
        }
        edges_self.push(Edge::new([vertices_self[vertices_self.len() - 1].clone(), self.vertices[1].clone()], self.curve.clone()));

        // Same story for edge_b
        edges_other.push(Edge::new([other.vertices[0].clone(), vertices_other[0].clone()], other.curve.clone()));
        for i in 0..vertices_other.len() - 1 {
            edges_other.push(Edge::new([vertices_other[i].clone(), vertices_other[i + 1].clone()], other.curve.clone()));
        }
        edges_other.push(Edge::new([vertices_other[vertices_other.len() - 1].clone(), other.vertices[1].clone()], other.curve.clone()));

        return (edges_self, edges_other);
    }
}
