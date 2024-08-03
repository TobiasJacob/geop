use std::collections::VecDeque;

use geop_geometry::{
    points::point::Point,
    surfaces::surface::{Surface, TangentPoint},
    EQ_THRESHOLD,
};
use geop_topology::topology::{face::Face, scene::Color};

use crate::{
    contour::rasterize_contour_into_line_list,
    edge_buffer::{EdgeBuffer, RenderEdge},
    triangle_buffer::{RenderTriangle, TriangleBuffer},
    vertex_buffer::RenderVertex,
    vertex_normal_buffer::RenderNormalVertex,
};

// struct PointBuffer {
//     vertices: Vec<RenderVertex>
// }

// struct RenderVertex {
//     position: [f32; 3],
//     color: [f32; 3]
// }

// Source: https://www.redblobgames.com/x/1842-delaunay-voronoi-sphere/
// To perform delaungy in 3d it is fine to project the points into their tangent plane and perform delaunay triangulation there.

// This function checks if the point is inside the circumcircle of the triangle. The points have to be in counter clockwise order.
pub fn inside_triangle_circumcircle(
    surface: &Surface,
    traingle: &RenderTriangle,
    point: RenderVertex,
) -> bool {
    // First project points into the tangent plane.
    let projected_triangles = [
        surface.log(traingle.a.into(), point.into()),
        surface.log(traingle.b.into(), point.into()),
        surface.log(traingle.c.into(), point.into()),
    ];
    // Now use the classic delaunay algorithm in 2d.

    // Matrix as in https://en.wikipedia.org/wiki/Delaunay_triangulation
    // For planar Delaunay triangulation, we will check if the point lies inside the circumcircle of the triangle

    let x1 = projected_triangles[0].0.x as f64;
    let y1 = projected_triangles[0].0.y as f64;
    let x2 = projected_triangles[1].0.x as f64;
    let y2 = projected_triangles[1].0.y as f64;
    let x3 = projected_triangles[2].0.x as f64;
    let y3 = projected_triangles[2].0.y as f64;

    let mat = [
        [x1, y1, (x1).powi(2) + (y1).powi(2)],
        [x2, y2, (x2).powi(2) + (y2).powi(2)],
        [x3, y3, (x3).powi(2) + (y3).powi(2)],
    ];

    // use rule of sarrus to calculate determinant
    let determinant = mat[0][0] * mat[1][1] * mat[2][2] - mat[0][2] * mat[1][1] * mat[2][0]
        + mat[0][1] * mat[1][2] * mat[2][0]
        - mat[0][1] * mat[1][0] * mat[2][2]
        + mat[0][2] * mat[1][0] * mat[2][1]
        - mat[0][0] * mat[1][2] * mat[2][1];

    // TODO: Check if we can do the triangulation in a single pass by choosing the point with minimum determinant right away...
    -determinant < -EQ_THRESHOLD // Check this is < -0.0001, which means it is inside the circumcircle
}

pub fn check_triangle_counter_clockwise(surface: &Surface, triangle: &RenderTriangle) -> bool {
    assert!(surface.on_surface(triangle.a.point()));
    assert!(surface.on_surface(triangle.b.point()));
    assert!(surface.on_surface(triangle.c.point()));

    let (v1, v2) = (
        surface.log(triangle.a.into(), triangle.b.into()),
        surface.log(triangle.a.into(), triangle.c.into()),
    );
    let det = v1.0.x * v2.0.y - v1.0.y * v2.0.x;
    return det > EQ_THRESHOLD; // Ignore if the triangle is colinear
}

fn edge_overlaps_edge(_surface: &Surface, edge: RenderEdge, other: RenderEdge) -> bool {
    if edge.start.point() == other.start.point()
        || edge.start.point() == other.end.point()
        || edge.end.point() == other.start.point()
        || edge.end.point() == other.end.point()
    {
        let dir1 = edge.end.point() - edge.start.point();
        let dir2 = other.end.point() - other.start.point();
        if dir1.is_parallel(dir2) {
            return true;
        }
        return false;
    }

    false
}

pub fn edge_intersects_contour(surface: &Surface, edge: &RenderEdge, contour: &EdgeBuffer) -> bool {
    for other in contour.edges.iter() {
        if edge_overlaps_edge(surface, *edge, *other) {
            return true;
        }
    }
    return false;
}

pub fn edge_intersects_contours(
    surface: &Surface,
    edge: &RenderEdge,
    contours: &[EdgeBuffer],
) -> bool {
    for contour in contours.iter() {
        if edge_intersects_contour(surface, edge, contour) {
            return true;
        }
    }
    return false;
}

pub fn triangle_intersects_triangle(
    surface: &Surface,
    triangle: &RenderTriangle,
    other_triangle: &RenderTriangle,
) -> bool {
    let reference_point = triangle.a;

    let project_fn = |p: &RenderNormalVertex| match surface {
        Surface::Plane(plane) => plane.log(reference_point.into(), p.point().into()).0,
        Surface::Sphere(sphere) => sphere.log(reference_point.into(), p.point().into()).0,
    };

    // six points with x and y coordinates
    let a1 = project_fn(&triangle.a);
    let b1 = project_fn(&triangle.b);
    let c1 = project_fn(&triangle.c);

    let a2 = project_fn(&other_triangle.a);
    let b2 = project_fn(&other_triangle.b);
    let c2 = project_fn(&other_triangle.c);

    // list of edges for both triangles
    let edges_triangle_1 = [(a1, b1), (b1, c1), (c1, a1)];
    let edges_triangle_2 = [(a2, b2), (b2, c2), (c2, a2)];

    // Use the separating axis theorem to check if the triangles intersect
    for edge in edges_triangle_1.iter().chain(edges_triangle_2.iter()) {
        let normal = Point::new(edge.1.y - edge.0.y, edge.0.x - edge.1.x, 0.0);
        let norm = normal.norm();
        if norm < EQ_THRESHOLD {
            continue;
        }
        let normal = normal / norm;

        let mut min_1 = f64::INFINITY;
        let mut max_1 = f64::NEG_INFINITY;
        let mut min_2 = f64::INFINITY;
        let mut max_2 = f64::NEG_INFINITY;

        for &point in [a1, b1, c1].iter() {
            let projected = normal.dot(point);
            min_1 = min_1.min(projected);
            max_1 = max_1.max(projected);
        }
        for &point in [a2, b2, c2].iter() {
            let projected = normal.dot(point);
            min_2 = min_2.min(projected);
            max_2 = max_2.max(projected);
        }
        // println!("Min 1: {}, Max 1: {}, Min 2: {}, Max 2: {}", min_1, max_1, min_2, max_2);
        // Proof that the triangles do not intersect
        if max_1 <= min_2 + EQ_THRESHOLD || max_2 <= min_1 + EQ_THRESHOLD {
            return false;
        }
    }

    true
}

pub fn triangle_intersects_triangle_list(
    surface: &Surface,
    triangle: &RenderTriangle,
    triangle_list: &[RenderTriangle],
) -> bool {
    for other_triangle in triangle_list {
        if triangle_intersects_triangle(surface, triangle, other_triangle) {
            return true;
        }
    }
    return false;
}

pub fn check_triangle(
    surface: &Surface,
    edge: RenderEdge,
    point: RenderVertex,
    color: Color,
    triangle_list: &[RenderTriangle],
    is_better_than: Option<&RenderTriangle>,
) -> Option<RenderTriangle> {
    let triangle = RenderTriangle::new(
        edge.start.into(),
        edge.end.into(),
        point.into(),
        color,
        surface.normal(edge.start.point()),
        surface.normal(edge.end.point()),
        surface.normal(point.point()),
    );
    // Check if the triangle is clockwise
    if !check_triangle_counter_clockwise(surface, &triangle) {
        return None;
    }

    if let Some(baseline_triangle) = is_better_than {
        if !inside_triangle_circumcircle(surface, baseline_triangle, point) {
            return None;
        }
    }
    // Sometimes, edges are pushed to the back of the queue which have already 2 triangles connected to them. In this case, we can skip the edge.
    if triangle_intersects_triangle_list(surface, &triangle, triangle_list) {
        return None;
    }
    return Some(triangle);
}

pub fn rasterize_face_into_triangle_list(face: &Face, color: Color) -> TriangleBuffer {
    println!("/////////////////////////////////////////////////////////");
    // Now we have to divide the face into triangles. First rasterize the boundaries. This will give us a set of open edges to work with
    let mut contours = Vec::<EdgeBuffer>::new();
    if let Some(boundary) = &face.boundary {
        contours.push(rasterize_contour_into_line_list(&boundary, color));
    }
    for contour in face.holes.iter() {
        let points = rasterize_contour_into_line_list(contour, color);
        for edge in points.edges.iter() {
            assert!(face.surface.on_surface(edge.start.point()));
            assert!(face.surface.on_surface(edge.end.point()));
        }
        contours.push(points);
    }

    let mut open_edges: VecDeque<RenderEdge> = contours
        .iter()
        .flat_map(|contour| contour.edges.iter())
        .cloned()
        .collect();
    let connection_points = contours
        .iter()
        .flat_map(|contour| contour.edges.iter())
        .map(|edge| edge.start)
        .collect::<Vec<RenderVertex>>();

    // TODO: Insert connection points in the surface
    // TODO: If open_edges has a length of 0, select a valid starting edge
    let mut triangles = Vec::<RenderTriangle>::new();

    // Now iterate until all open_edges are processed.
    while let Some(edge) = open_edges.pop_front() {
        let mut best_triangle: Option<(RenderVertex, RenderTriangle)> = None;
        // Now find the smallest valid triangle
        loop {
            let mut found_better_triangle = false;
            for point in connection_points.iter() {
                if let Some(triangle) = check_triangle(
                    &face.surface,
                    edge,
                    *point,
                    color,
                    &triangles,
                    best_triangle.as_ref().map(|(_, t)| t),
                ) {
                    found_better_triangle = true;
                    best_triangle = Some((*point, triangle));
                    break;
                }
            }
            if !found_better_triangle {
                break;
            }
        }
        if let Some((point, best_triangle)) = best_triangle {
            triangles.push(best_triangle);

            for inner_edge in [
                RenderEdge::new(edge.start.into(), point.into(), color),
                RenderEdge::new(point.into(), edge.end.into(), color),
            ] {
                if !edge_intersects_contours(&face.surface, &inner_edge, &contours) {
                    open_edges.push_back(inner_edge);
                }
            }
        }
    }

    return TriangleBuffer::new(triangles);
}

pub fn rasterize_face_into_line_list(face: &Face, color: Color) -> EdgeBuffer {
    let mut buffer = EdgeBuffer::empty();
    if let Some(boundary) = &face.boundary {
        buffer.join(&rasterize_contour_into_line_list(boundary, color));
    }
    for contour in face.holes.iter() {
        buffer.join(&rasterize_contour_into_line_list(contour, color));
    }
    buffer
}
