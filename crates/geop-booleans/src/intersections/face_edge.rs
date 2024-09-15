use geop_geometry::{
    curve_surface_intersection::curve_surface::{
        curve_surface_intersection, CurveSurfaceIntersection,
    },
    curves::CurveLike,
    points::point::Point,
};

use geop_topology::{
    contains::face_point::{face_point_contains, FacePointContains},
    topology::{edge::Edge, face::Face},
};

use super::edge_edge::{edge_edge_intersection, EdgeEdgeIntersection};

pub enum FaceEdgeIntersection {
    None,
    Points(Vec<Point>),
    Edges(Vec<Edge>),
}

pub fn face_edge_intersection(face: &Face, edge: &Edge) -> FaceEdgeIntersection {
    match curve_surface_intersection(&edge.curve, &face.surface) {
        CurveSurfaceIntersection::Points(mut points) => FaceEdgeIntersection::Points(
            points
                .drain(..)
                .filter(|p| face_point_contains(face, *p) == FacePointContains::Inside)
                .collect(),
        ),
        CurveSurfaceIntersection::Curve(curve) => {
            let mut points = Vec::<Option<Point>>::new();
            points.push(edge.start);
            points.push(edge.end);

            for e in face.all_edges().iter() {
                match edge_edge_intersection(edge, e) {
                    EdgeEdgeIntersection::Edges(es) => {
                        for e in es.iter() {
                            points.push(e.start);
                            points.push(e.end);
                        }
                    }
                    EdgeEdgeIntersection::Points(ps) => {
                        for p in ps.iter() {
                            points.push(Some(*p));
                        }
                    }
                    EdgeEdgeIntersection::None => {}
                }
            }

            // Now sort the points and remove duplicates
            let mut points = curve.sort(points);
            points.dedup();
            points.push(None); // Add a None to the end to close the loop

            let mut edges = Vec::<Edge>::new();
            for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
                let m = curve.get_midpoint(*p1, *p2);
                if face_point_contains(face, m) == FacePointContains::Inside {
                    edges.push(Edge::new(*p1, *p2, curve.clone()));
                }
            }

            return FaceEdgeIntersection::Edges(edges);
        }
        CurveSurfaceIntersection::None => FaceEdgeIntersection::None,
    }
}
