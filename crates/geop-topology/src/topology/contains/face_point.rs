use std::rc::Rc;

use geop_geometry::points::point::Point;

use crate::topology::{edge::Edge, face::Face, intersections::{edge_edge::EdgeEdgeIntersection, contour_edge::countour_edge_intersection_points}};

use super::edge_point::{EdgeContains, edge_contains_point};


#[derive(Clone, Debug, PartialEq)]
pub enum FaceContainsPoint {
    Inside,
    OnEdge(Rc<Edge>),
    OnPoint(Rc<Point>),
    Outside,
}


pub fn face_contains_point(face: &Face, point: Point) -> FaceContainsPoint {
    // println!("face_contains_point: {:?}", point);
    // println!("face: {:}", face);
    // If the point is on the border, it is part of the set
    for edge in face.all_edges() {
        match edge_contains_point(&edge, point) {
            EdgeContains::Inside => return FaceContainsPoint::OnEdge(edge.clone()),
            EdgeContains::OnPoint(point) => return FaceContainsPoint::OnPoint(point),
            EdgeContains::Outside => continue,
        }
    }
    // Draw a line from the point to a random point on the border.
    let q: Point = *face.boundaries[0].edges[0].start;
    let curve = face.edge_from_to(Rc::new(point), Rc::new(q));

    // Find the closest intersection point and check by using the face normal and the curve tangent if the intersection is from inside or outside.
    let mut closest_distance = face.surface.surface().distance(point, q);
    let curve_dir = curve.tangent(q);
    let normal = face.surface.surface().normal(q);
    let contour_dir = face.boundaries[0].tangent(q);
    let mut closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);
    
    for int in countour_edge_intersection_points(face, &*curve) {
        // println!("int: {:?}", int);
        let distance = face.surface.surface().distance(point, *int);
        if distance < closest_distance {
            let curve_dir = curve.tangent(*int);
            let normal = face.surface.surface().normal(*int);
            let contour_dir = face.boundary_tangent(*int);
            closest_distance = distance;
            closest_intersect_from_inside = contour_dir.is_inside(normal, curve_dir);
        }
    }

    match closest_intersect_from_inside {
        true => FaceContainsPoint::Inside,
        false => FaceContainsPoint::Outside,
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use geop_geometry::{points::point::Point, surfaces::plane::Plane};

    use crate::topology::{edge::Edge, face::{Face, face_surface::FaceSurface}, contour::Contour, contains::face_point::{face_contains_point, FaceContainsPoint}};


    // #[test]
    // fn test_inside_outside() {
    //     let p1 = Rc::new(Point::new(0.0, 0.0, 0.0));
    //     let p2 = Rc::new(Point::new(1.0, 0.0, 0.0));
    //     let p3 = Rc::new(Point::new(1.0, 1.0, 0.0));
    //     let p4 = Rc::new(Point::new(0.0, 1.0, 0.0));

    //     let normal = Rc::new(Point::new(0.0, 0.0, 1.0));

    //     let e1 = Rc::new(Edge::new_line(p1.clone(), p2.clone()));
    //     let e2 = Rc::new(Edge::new_line(p2.clone(), p3.clone()));
    //     let e3 = Rc::new(Edge::new_line(p3.clone(), p4.clone()));
    //     let e4 = Rc::new(Edge::new_line(p4.clone(), p1.clone()));

    //     let contour = Contour::new(vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()]);

    //     assert!(contour.tangent(*p1).is_inside(*normal, Point::new(-1.0, -1.0, 0.0)));
    //     assert!(!contour.tangent(*p3).is_inside(*normal, Point::new(-1.0, -1.0, 0.0)));

    //     let face = Face::new(vec![contour], Rc::new(FaceSurface::Plane(Plane::new(*p1, *p2 - *p1, *p3 - *p1))));
    //     assert!(face_contains_point(&face, Point::new(0.5, 0.5, 0.0)) == FaceContainsPoint::Inside);
    // }
}
