use std::f32::consts::E;

use geop_geometry::{points::point::Point, surfaces::{surface::{Surface, TangentPoint}, sphere::Sphere, plane::Plane}, EQ_THRESHOLD};
use geop_topology::topology::{face::{Face, FaceSurface}, edge};

use crate::{vertex_buffer::{VertexBuffer, RenderVertex}, contour::rasterize_contour_into_line_list, triangle_buffer::{TriangleBuffer, RenderTriangle}, edge_buffer::{EdgeBuffer, RenderEdge}};

// struct VertexBuffer {
//     vertices: Vec<RenderVertex>
// }

// struct RenderVertex {
//     position: [f32; 3],
//     color: [f32; 3]
// }

pub enum DelaunaySurface {
    Sphere(Sphere),
    Plane(Plane),
}

impl DelaunaySurface {
    pub fn new(face: &Face) -> DelaunaySurface {
        match &*face.surface {
            FaceSurface::Sphere(sphere) => DelaunaySurface::Sphere(sphere.clone()),
            FaceSurface::Plane(plane) => DelaunaySurface::Plane(plane.clone()),
        }
    }
}

// Source: https://www.redblobgames.com/x/1842-delaunay-voronoi-sphere/
// To perform delaungy in 3d it is fine to project the points into their tangent plane and perform delaunay triangulation there.

// This function checks if the point is inside the circumcircle of the triangle. The points have to be in counter clockwise order.
pub fn inside_triangle_circumcircle(surface: &DelaunaySurface, traingle: &[RenderVertex; 3], point: RenderVertex) -> f64 {
    // First project points into the tangent plane.
    let projected_triangles = match surface {
        DelaunaySurface::Plane(plane) => {
            traingle.iter().map(|p| plane.log(point.into(), (*p).into())).collect::<Vec<TangentPoint>>()
        },
        DelaunaySurface::Sphere(sphere) => {
            traingle.iter().map(|p| sphere.log(point.into(), (*p).into())).collect::<Vec<TangentPoint>>()
        }
    };
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
    let determinant = mat[0][0] * mat[1][1] * mat[2][2] 
        - mat[0][2] * mat[1][1] * mat[2][0] 
        + mat[0][1] * mat[1][2] * mat[2][0] 
        - mat[0][1] * mat[1][0] * mat[2][2] 
        + mat[0][2] * mat[1][0] * mat[2][1] 
        - mat[0][0] * mat[1][2] * mat[2][1];

    determinant // Check this is < -0.0001, which means it is inside the circumcircle
}

pub fn check_triangle_counter_clockwise(surface: &DelaunaySurface, traingle: &[RenderVertex; 3]) -> bool {
    let (v1, v2) = match surface {
        DelaunaySurface::Plane(plane) => {
            (plane.log(traingle[0].into(), traingle[1].into()), plane.log(traingle[0].into(), traingle[2].into()))
        },
        DelaunaySurface::Sphere(sphere) => {
            (sphere.log(traingle[0].into(), traingle[1].into()), sphere.log(traingle[0].into(), traingle[2].into()))
        }
    };
    let det = v1.0.x * v2.0.y - v1.0.y * v2.0.x;
    return det > EQ_THRESHOLD; // Ignore if the triangle is colinear
}

pub fn triangle_intersects_triangle(surface: &DelaunaySurface, triangle: &RenderTriangle, other_triangle: &RenderTriangle) -> bool {
    let reference_point = triangle.a;
    
    let project_fn = |p: RenderVertex| match surface {
        DelaunaySurface::Plane(plane) => plane.log(reference_point.into(), p.into()).0,
        DelaunaySurface::Sphere(sphere) => sphere.log(reference_point.into(), p.into()).0,
    };

    // six points with x and y coordinates
    let a1 = project_fn(triangle.a);
    let b1 = project_fn(triangle.b);
    let c1 = project_fn(triangle.c);

    let a2 = project_fn(other_triangle.a);
    let b2 = project_fn(other_triangle.b);
    let c2 = project_fn(other_triangle.c);

    // list of edges for both triangles
    let edges_triangle_1 = [(a1, b1), (b1, c1), (c1, a1)];
    let edges_triangle_2 = [(a2, b2), (b2, c2), (c2, a2)];

    // Use the separating axis theorem to check if the triangles intersect
    for edge in edges_triangle_1.iter().chain(edges_triangle_2.iter()) {
        let normal = [(edge.1).y - (edge.0).y, (edge.0).x - (edge.1).x];

        let mut min_1 = f64::INFINITY;
        let mut max_1 = f64::NEG_INFINITY;
        let mut min_2 = f64::INFINITY;
        let mut max_2 = f64::NEG_INFINITY;

        for &point in [a1, b1, c1].iter() {
            let projected = normal[0] * point.x + normal[1] * point.y;
            min_1 = min_1.min(projected);
            max_1 = max_1.max(projected);
        }
        for &point in [a2, b2, c2].iter() {
            let projected = normal[0] * point.x + normal[1] * point.y;
            min_2 = min_2.min(projected);
            max_2 = max_2.max(projected);
        }
        if max_1 < min_2 + EQ_THRESHOLD || max_2 < min_1 + EQ_THRESHOLD {
            return false;
        }
    }

    true
}

pub fn triangle_intersects_triangle_list(surface: &DelaunaySurface, triangle: &RenderTriangle, triangle_list: &[RenderTriangle]) -> bool {
    for other_triangle in triangle_list {
        if triangle_intersects_triangle(surface, triangle, other_triangle) {
            return true;
        }
    }
    return false;
}

pub fn rasterize_face_into_triangle_list(face: &Face, color: [f32; 3]) -> TriangleBuffer {
    println!("/////////////////////////////////////////////////////////");
    let surface = DelaunaySurface::new(face);
    // Now we have to divide the face into triangles. First rasterize the boundaries.
    let mut contours = Vec::<EdgeBuffer>::new();
    for contour in face.boundaries.iter() {
        let points = rasterize_contour_into_line_list(contour, color);
        contours.push(points);
    }

    let open_edges: Vec<RenderEdge> = contours.iter().flat_map(|contour| contour.edges.iter()).cloned().collect();
    let mut triangles = Vec::<RenderTriangle>::new();

    // Now make sure that all discrete boundaries are connected to a single boundary.
    for edge in open_edges.iter() {
        let mut i = usize::MAX;
        println!("Render Edge: {:?}", edge);
        for j in 0..open_edges.len() {
            let point = open_edges[j].start;
            if !check_triangle_counter_clockwise(&surface, &[edge.start, edge.end, point]) {
                continue;
            }
            if triangle_intersects_triangle_list(&surface, &RenderTriangle::new(edge.start.into(), edge.end.into(), point.into(), color), &triangles) {
                continue;
            }
            i = j;
            break;
        }
        if i == usize::MAX {
            continue;
        }
        println!("Found start i: {:?}", i);
        loop {
            let mut found_one_inside = false;
            let mut min_det = 0.0;
            for j in 0..open_edges.len() {
                let current_point = open_edges[i].start;
                let new_point = open_edges[j].start;
                if i == j || current_point.point() == new_point.point() {
                    continue;
                }
                if !check_triangle_counter_clockwise(&surface, &[edge.start, edge.end, new_point]) {
                    continue;
                }
                let new_det = inside_triangle_circumcircle(&surface, &[edge.start, edge.end, current_point], new_point);
                if new_det >= min_det {
                    continue;
                }
                if triangle_intersects_triangle_list(&surface, &RenderTriangle::new(edge.start.into(), edge.end.into(), new_point.into(), color), &triangles) {
                    continue;
                }
                i = j;
                found_one_inside = true;
                min_det = new_det;
                println!("{:?} to {:?}, i = {:?}, new_point = {:?}", edge, current_point, i, new_point);
            }
            if !found_one_inside {
                break;
            }
        }
        let point = open_edges[i].start;
        triangles.push(RenderTriangle::new(edge.start.into(), edge.end.into(), point.into(), color));
    }

    return TriangleBuffer::new(triangles);
}