use std::collections::VecDeque;

use geop_geometry::{points::point::Point, surfaces::surface::Surface, EQ_THRESHOLD};
use geop_topology::{
    contains::face_point::{face_point_contains, FacePointContains},
    topology::{face::Face, scene::Color},
};

use crate::{
    contour::rasterize_contour_into_line_list,
    edge_buffer::{EdgeBuffer, RenderEdge},
    triangle_buffer::{RenderTriangle, TriangleBuffer},
    vertex_buffer::{RenderVertex, VertexBuffer},
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
    edge: &RenderEdge,
    ref_point: &RenderVertex,
    point: &RenderVertex,
) -> bool {
    let mid_point = surface.project(edge.mid_point());
    // First project points into the tangent plane.
    let projected_triangle0 = surface.log(mid_point, edge.start.point());
    let projected_triangle1 = surface.log(mid_point, edge.end.point());
    let projected_triangle2 = surface.log(mid_point, ref_point.point());
    let point = surface.log(mid_point, point.point());

    let (projected_triangle0, projected_triangle1, projected_triangle2, point) = match (
        projected_triangle0,
        projected_triangle1,
        projected_triangle2,
        point,
    ) {
        (Some(v0), Some(v1), Some(v2), Some(v3)) => (v0, v1, v2, v3),
        _ => return false,
    };

    // println!("Projected triangle 0: {:?}", projected_triangle0);
    // println!("Projected triangle 1: {:?}", projected_triangle1);
    // println!("Projected triangle 2: {:?}", projected_triangle2);
    // println!("Point: {:?}", point);

    let projected_triangle0 = projected_triangle0 - point;
    let projected_triangle1 = projected_triangle1 - point;
    let projected_triangle2 = projected_triangle2 - point;

    // Now use the classic delaunay algorithm in 2d.

    // Matrix as in https://en.wikipedia.org/wiki/Delaunay_triangulation
    // For planar Delaunay triangulation, we will check if the point lies inside the circumcircle of the triangle
    let cross_dir = surface.normal(mid_point);

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
    det > EQ_THRESHOLD * 10.0 // Check this is -det < -0.0001, which means it is inside the circumcircle
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

pub fn edge_will_be_blocked_by_contour(edge: &RenderEdge, contours: &[EdgeBuffer]) -> bool {
    let flipped_edge = edge.flip();
    for contour in contours.iter() {
        for e in contour.edges.iter() {
            if edge == e || flipped_edge == *e {
                return true;
            }
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
    let a1 = surface.log(reference_point, triangle.a.point());
    let b1 = surface.log(reference_point, triangle.b.point());
    let c1 = surface.log(reference_point, triangle.c.point());

    let (a1, b1, c1) = match (a1, b1, c1) {
        (Some(a1), Some(b1), Some(c1)) => (a1, b1, c1),
        _ => return true,
    };

    let a2 = surface.log(reference_point, other_triangle.a.point());
    let b2 = surface.log(reference_point, other_triangle.b.point());
    let c2 = surface.log(reference_point, other_triangle.c.point());

    let (a2, b2, c2) = match (a2, b2, c2) {
        (Some(a2), Some(b2), Some(c2)) => (a2, b2, c2),
        _ => return true,
    };

    // list of edges for both triangles
    let edges_triangle_1 = [(a1, b1), (b1, c1), (c1, a1)];
    let edges_triangle_2 = [(a2, b2), (b2, c2), (c2, a2)];

    // Use the separating axis theorem to check if the triangles intersect
    for edge in edges_triangle_1.iter().chain(edges_triangle_2.iter()) {
        let project_axis = surface.normal(reference_point).cross(edge.1 - edge.0);
        let norm = project_axis.norm();
        if norm < EQ_THRESHOLD {
            continue;
        }
        let normal = project_axis / norm;

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

// 3D line line intersection
pub fn render_edge_intersects_render_edge(edge1: &RenderEdge, edge2: &RenderEdge) -> bool {
    let dir1 = edge1.end.point() - edge1.start.point();
    let dir2 = edge2.end.point() - edge2.start.point();

    // Parallel lines are assumed to not intersect
    let cross = dir1.cross(dir2);
    let cross_normsq = cross.norm_sq();
    if cross_normsq < EQ_THRESHOLD {
        return false;
    }

    let diff = edge2.start.point() - edge1.start.point();
    let t = diff.cross(dir2).dot(cross) / cross_normsq;
    let u = diff.cross(dir1).dot(cross) / cross_normsq;

    // Check if the intersection point is on the line segments
    if t > EQ_THRESHOLD && t < 1.0 - EQ_THRESHOLD && u > EQ_THRESHOLD && u < 1.0 - EQ_THRESHOLD {
        return true;
    }
    return false;
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
    processed_edges: &[RenderEdge],
    other_candidate_points: &[RenderVertex],
) -> Option<()> {
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

    // let mut violation_count = 0 as usize;
    for other_point in other_candidate_points.iter() {
        if inside_triangle_circumcircle(surface, &edge, &point, other_point) {
            // violation_count += 1;
            return None;
        }
        // println!("Detected inside_triangle_circumcircle");
        // println!("Edge: {:?}", edge);
        // println!("Point: {:?}", point);
    }
    // Make sure that in cases where the delaunay triangulation is not clear, we do not create a triangle that intersects with the existing triangles
    // if triangle_intersects_triangle_list(surface, &triangle, triangle_list) {
    //     return None;
    // }
    if processed_edges.contains(&RenderEdge::new(edge.end.into(), point.into(), color)) {
        return None;
    }
    if processed_edges.contains(&RenderEdge::new(point.into(), edge.start.into(), color)) {
        return None;
    }
    // Check if the triangle intersects with any boundary. This prevents an accidential "wall glitch" of small long thin holes in a circle, which may be overstepped otherwise.
    // let new_edge1 = RenderEdge::new(edge.start.into(), point.into(), color);
    // let new_edge2 = RenderEdge::new(point.into(), edge.end.into(), color);
    // for contour in contours.iter() {
    //     for e in contour.edges.iter() {
    //         if render_edge_intersects_render_edge(&new_edge1, e) {
    //             return None;
    //         }
    //         if render_edge_intersects_render_edge(&new_edge2, e) {
    //             return None;
    //         }
    //     }
    // }
    // for processed_edge in processed_edges.iter() {
    //     if render_edge_intersects_render_edge(&new_edge1, processed_edge) {
    //         return None;
    //     }
    //     if render_edge_intersects_render_edge(&new_edge2, processed_edge) {
    //         return None;
    //     }
    // }
    // for other_triangle in triangles.iter() {
    //     if triangle_intersects_triangle(surface, &triangle, other_triangle) {
    //         return None;
    //     }
    // }
    return Some(());
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
            .point_grid(1.0)
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
            open_edges.push_back(best_edge.flip());
            open_edges.push_back(best_edge);
        }
    }

    let mut triangles = Vec::<RenderTriangle>::new();

    // Now iterate until all open_edges are processed.
    let mut processed_edges = Vec::<RenderEdge>::new();
    let mut counter = 0;
    while let Some(edge) = open_edges.pop_front() {
        if processed_edges.contains(&edge) {
            continue;
        }
        processed_edges.push(edge);

        println!(
            "Counter: {} Open edges: {} Result len: {}",
            counter,
            open_edges.len(),
            triangles.len()
        );
        counter += 1;
        if counter > 1500 {
            break;
        }
        let mut best_triangle_point: Option<RenderVertex> = None;
        // let mut best_value = usize::max_value();
        // Now find the best valid triangle
        for point in connection_points.iter() {
            if let Some(_) = check_triangle(
                &face.surface,
                edge,
                *point,
                color,
                processed_edges.as_slice(),
                &connection_points,
            ) {
                best_triangle_point = Some(*point);
                break;
            }
        }
        if let Some(point) = best_triangle_point {
            triangles.push(RenderTriangle::new(
                edge.start.into(),
                edge.end.into(),
                point.into(),
                color,
                face.surface.normal(edge.start.point()),
                face.surface.normal(edge.end.point()),
                face.surface.normal(point.point()),
            ));
            processed_edges.push(RenderEdge::new(point.into(), edge.start.into(), color));
            processed_edges.push(RenderEdge::new(edge.end.into(), point.into(), color));

            for inner_edge in [
                RenderEdge::new(edge.start.into(), point.into(), color),
                RenderEdge::new(point.into(), edge.end.into(), color),
            ] {
                // This will prevent the algorithm from spreading out of the face and filling the holes
                if !edge_will_be_blocked_by_contour(&inner_edge, &contours) {
                    if !open_edges.contains(&inner_edge) {
                        open_edges.push_front(inner_edge);
                    }
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

    for p in face.all_points().iter() {
        buffer.push(RenderVertex::new(p.clone(), color));
    }
    VertexBuffer::new(buffer)
}
