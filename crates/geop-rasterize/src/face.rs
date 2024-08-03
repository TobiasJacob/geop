use std::collections::VecDeque;

use geop_geometry::{
    points::point::Point,
    surfaces::surface::{Surface, TangentPoint},
    EQ_THRESHOLD,
};
use geop_topology::{
    contains::face_point::{face_point_contains, FacePointContains},
    topology::{face::Face, scene::Color},
};

use crate::{
    contour::rasterize_contour_into_line_list,
    edge_buffer::{EdgeBuffer, RenderEdge},
    triangle_buffer::{RenderTriangle, TriangleBuffer},
    vertex_buffer::{RenderVertex, VertexBuffer},
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

// pub fn inside_triangle_circumcircle(
//     surface: &Surface,
//     edge: &RenderEdge,
//     ref_point: &RenderVertex,
//     point: &RenderVertex,
// ) -> bool {
//     // First project points into the tangent plane.
//     // Using mid_point as the reference point will ensure consistent results of the projection if ref_point is changed for point.
//     let mid_point = surface.project(edge.mid_point());
//     let projected_triangles = [
//         surface.log(edge.start.into(), mid_point),
//         surface.log(edge.end.into(), mid_point),
//         surface.log(ref_point.point(), mid_point)
//     ];

fn determinant(row0: Point, row1: Point, row2: Point) -> f64 {
    // use rule of sarrus to calculate determinant
    row0.x * row1.y * row2.z - row0.z * row1.y * row2.x + row0.y * row1.z * row2.x
        - row0.y * row1.x * row2.z
        + row0.z * row1.x * row2.y
        - row0.x * row1.z * row2.y
}

// This function checks if the point is inside the circumcircle of the triangle. The points have to be in counter clockwise order.
pub fn inside_triangle_circumcircle(
    surface: &Surface,
    traingle: &RenderTriangle,
    point: RenderVertex,
) -> bool {
    // First project points into the tangent plane.
    let projected_triangle0 = surface.log(traingle.a.point(), point.point());
    let projected_triangle1 = surface.log(traingle.b.point(), point.point());
    let projected_triangle2 = surface.log(traingle.c.point(), point.point());

    let (projected_triangle0, projected_triangle1, projected_triangle2) = match (
        projected_triangle0,
        projected_triangle1,
        projected_triangle2,
    ) {
        (Some(p0), Some(p1), Some(p2)) => (p0, p1, p2),
        _ => return false,
    };

    // Now use the classic delaunay algorithm in 2d.

    // Matrix as in https://en.wikipedia.org/wiki/Delaunay_triangulation
    // For planar Delaunay triangulation, we will check if the point lies inside the circumcircle of the triangle
    let cross_dir = surface.normal(point.point());

    let mat0 = projected_triangle0 + cross_dir * projected_triangle0.norm_sq();
    let mat1 = projected_triangle1 + cross_dir * projected_triangle1.norm_sq();
    let mat2 = projected_triangle2 + cross_dir * projected_triangle2.norm_sq();
    let det = determinant(mat0, mat1, mat2);

    // TODO: Check if we can do the triangulation in a single pass by choosing the point with minimum determinant right away...
    // Check if -determinant is < 0, which means it is inside the circumcircle
    // println!("Determinant: {}", det);
    // Check if determinant is nan
    if det.is_nan() {
        panic!("Determinant is nan");
    }
    det > EQ_THRESHOLD * 10.0 // Check this is < -0.0001, which means it is inside the circumcircle
}

pub fn check_triangle_counter_clockwise(surface: &Surface, triangle: &RenderTriangle) -> bool {
    assert!(surface.on_surface(triangle.a.point()));
    assert!(surface.on_surface(triangle.b.point()));
    assert!(surface.on_surface(triangle.c.point()));

    let (v1, v2) = (
        surface.log(triangle.a.into(), triangle.b.into()),
        surface.log(triangle.a.into(), triangle.c.into()),
    );

    let (v1, v2) = match (v1, v2) {
        (Some(v1), Some(v2)) => (v1, v2),
        _ => return false,
    };

    let normal = surface.normal(triangle.a.point());
    let det = determinant(v1, v2, normal);
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
    let reference_point = triangle.a.point();

    // six points with x and y coordinates
    let a1 = surface.log(triangle.a.point(), reference_point);
    let b1 = surface.log(triangle.b.point(), reference_point);
    let c1 = surface.log(triangle.c.point(), reference_point);

    let (a1, b1, c1) = match (a1, b1, c1) {
        (Some(a1), Some(b1), Some(c1)) => (a1, b1, c1),
        _ => return true,
    };

    let a2 = surface.log(other_triangle.a.point(), reference_point);
    let b2 = surface.log(other_triangle.b.point(), reference_point);
    let c2 = surface.log(other_triangle.c.point(), reference_point);

    let (a2, b2, c2) = match (a2, b2, c2) {
        (Some(a2), Some(b2), Some(c2)) => (a2, b2, c2),
        _ => return true,
    };

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
    // Check if the triangle is clockwise and has area > 0
    if !check_triangle_counter_clockwise(surface, &triangle) {
        return None;
    }

    if let Some(baseline_triangle) = is_better_than {
        if !inside_triangle_circumcircle(surface, baseline_triangle, point) {
            return None;
        }
        println!("Detected inside_triangle_circumcircle");
        println!(
            "Triangle: {:?} {:?} {:?}",
            baseline_triangle.a.point(),
            baseline_triangle.b.point(),
            baseline_triangle.c.point()
        );
        println!(
            "Triangle-log: {:?} {:?} {:?}",
            surface.log(baseline_triangle.a.point(), point.into()),
            surface.log(baseline_triangle.b.point(), point.into()),
            surface.log(baseline_triangle.c.point(), point.into())
        );
        println!("Point: {:?}", point);
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

    // Rasterize the edges first
    let mut open_edges: VecDeque<RenderEdge> = contours
        .iter()
        .flat_map(|contour| contour.edges.iter())
        .cloned()
        .collect();
    let mut connection_points = contours
        .iter()
        .flat_map(|contour| contour.edges.iter())
        .map(|edge| edge.start)
        .collect::<Vec<RenderVertex>>();

    // Then generate additional points on the surface
    connection_points.extend(
        face.surface
            .point_grid()
            .drain(..)
            .filter(|p| face_point_contains(face, *p) == FacePointContains::Inside)
            .map(|point| RenderVertex::new(point.clone(), color)),
    );

    // If the surface is unbounded, we have to find a valid edge to start with. From there we can build the triangles.
    if open_edges.is_empty() {
        // Find the two closest points
        let mut min_distance = f64::INFINITY;
        let mut best_edge = None;
        for i in 0..connection_points.len() {
            for j in i + 1..connection_points.len() {
                let distance = face
                    .surface
                    .distance(connection_points[i].point(), connection_points[j].point());
                if distance < EQ_THRESHOLD {
                    continue;
                }
                if distance < min_distance {
                    min_distance = distance;
                    best_edge = Some(RenderEdge::new(
                        connection_points[i].into(),
                        connection_points[j].into(),
                        color,
                    ));
                }
            }
        }
        if let Some(best_edge) = best_edge {
            open_edges.push_back(best_edge);
        }
    }

    let mut triangles = Vec::<RenderTriangle>::new();

    // Now iterate until all open_edges are processed.
    while let Some(edge) = open_edges.pop_front() {
        let mut best_triangle: Option<(RenderVertex, RenderTriangle)> = None;
        println!("Open edges: {}", open_edges.len());
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
                    println!("Found triangle: {:?}", point);
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

pub fn rasterize_face_into_vertex_list(face: &Face, color: Color) -> VertexBuffer {
    let mut buffer = Vec::<RenderVertex>::new();
    if let Some(boundary) = &face.boundary {
        buffer.extend(
            rasterize_contour_into_line_list(boundary, color)
                .edges
                .iter()
                .map(|edge| edge.start),
        );
    }
    for contour in face.holes.iter() {
        buffer.extend(
            rasterize_contour_into_line_list(contour, color)
                .edges
                .iter()
                .map(|edge| edge.start),
        );
    }
    VertexBuffer::new(buffer)
}
