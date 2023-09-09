use geop_geometry::{points::point::Point, surfaces::{surface::{Surface, TangentPoint}, sphere::Sphere, plane::Plane}};
use geop_topology::topology::face::Face;

use crate::{vertex_buffer::VertexBuffer, contour::rasterize_contour_into_line_list, triangle_buffer::TriangleBuffer, edge_buffer::EdgeBuffer};

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

// Source: https://www.redblobgames.com/x/1842-delaunay-voronoi-sphere/
// To perform delaungy in 3d it is fine to project the points into their tangent plane and perform delaunay triangulation there.

// This function checks if the point is inside the circumcircle of the triangle. The points have to be in counter clockwise order.
pub fn check_delaunay_triangulation(surface: &DelaunaySurface, traingle: &[Point; 3], point: Point) -> bool {
    match surface {
        DelaunaySurface::Plane(plane) => {
            let projected_triangle = traingle.iter().map(|p| plane.log(point, *p)).collect::<Vec<TangentPoint>>();
            // Matrix as in https://en.wikipedia.org/wiki/Delaunay_triangulation
            // For planar Delaunay triangulation, we will check if the point lies inside the circumcircle of the triangle
            // let mat = na::Matrix4::new(
            //     triangle[0].x - point.x, triangle[0].y - point.y, (triangle[0].x - point.x).powi(2) + (triangle[0].y - point.y).powi(2), 1.0,
            //     triangle[1].x - point.x, triangle[1].y - point.y, (triangle[1].x - point.x).powi(2) + (triangle[1].y - point.y).powi(2), 1.0,
            //     triangle[2].x - point.x, triangle[2].y - point.y, (triangle[2].x - point.x).powi(2) + (triangle[2].y - point.y).powi(2), 1.0,
            //     0.0, 0.0, 0.0, 1.0,
            // );
            // mat.determinant() <= 0.0;

            let p1 = projected_triangle[0].0;
            let p2 = projected_triangle[1].0;
            let p3 = projected_triangle[2].0;
            let p4 = point;
            let a = p1.x - p4.x;
            let b = p1.y - p4.y;
            let c = p2.x - p4.x;
            let d = p2.y - p4.y;
            let e = p3.x - p4.x;
            let f = p3.y - p4.y;
            let g = a * (p1.x + p4.x) + b * (p1.y + p4.y);
            let h = c * (p2.x + p4.x) + d * (p2.y + p4.y);
            let i = e * (p3.x + p4.x) + f * (p3.y + p4.y);
            let det = a * (d * i - e * h) - b * (c * i - e * g) + c * (b * i - d * g);
            return det > 0.0;
        },
        DelaunaySurface::Sphere(sphere) => {
            todo!("Check if the triangulation is delaunay")

        }
    }
}

pub fn rasterize_face_into_triangle_list(face: &Face, color: [f32; 3]) -> TriangleBuffer {
    // Now we have to divide the face into triangles. First rasterize the boundaries.
    let mut vertices = Vec::<EdgeBuffer>::new();
    for contour in face.boundaries.iter() {
        let points = rasterize_contour_into_line_list(contour, color);
        vertices.push(points);
    }
    
    // Now do a delaunay triangulation on the points while making sure that triangles lie within the face.
    // The inside of the face is intersection of all counter clockwise boundaries.
    
    
    todo!("Rasterize the interior of the face")
}