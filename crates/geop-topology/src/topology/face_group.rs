// use std::rc::Rc;

// use geop_geometry::geometry::points::point::Point;

// use super::{Face::Face, edge::{EdgeLoop::EdgeLoop}, Vertex};

use std::rc::Rc;

use geop_geometry::points::point::Point;

use super::{face::{Face, FaceIntersection}, edge_loop::EdgeLoop, vertex::Vertex, edge::Edge};


// A watertight group of faces.
pub struct FaceGroup {
    pub faces: Vec<Rc<Face>>
}

impl FaceGroup {
    pub fn new(faces: Vec<Rc<Face>>) -> FaceGroup {
        FaceGroup {
            faces
        }
    }

    pub fn contains(&self, other: &Point) -> bool {
        todo!("Implement contains");
    }

    pub fn subsurface(&self, edge_loops: &Vec<EdgeLoop>) -> Vec<Vec<Rc<Face>>> {
        let result = Vec::<Rc<Face>>::new();
        for face in &self.faces {
            let mut face = face.clone();
            for edge_loop in edge_loops {
                face = face.split_if_necessary(edge_loop);
            }
            result.push(face);
        }
        result
    }

    pub fn intersect(&self, other: &FaceGroup) -> Vec<EdgeLoop> {
        let mut face_intersections: Vec<FaceIntersection> = Vec::new();

        for face in &self.faces {
            for other_face in &other.faces {
                let intersections = face.intersect(other_face);
                if intersections.len() > 0 {
                    face_intersections.extend(intersections);
                }
            }
        }

        let result: Vec<EdgeLoop> = Vec::new();
        let edges: Vec<Edge> = Vec::new();

        for intersect in face_intersections {
            match intersect {
                FaceIntersection::Face(face) => {
                    result.extend(face.boundaries);
                },
                FaceIntersection::EdgeLoop(edge_loop) => {
                    result.push(edge_loop);
                },
                FaceIntersection::Edge(edge) => {
                    // Needs to be puzzeled back together
                    edges.push(edge);
                },
                FaceIntersection::Vertex(vertex) => {
                    // Ignore
                },
            }
        }

        let mut edge_loop = Vec::<Edge>::new();
        while let Some(edge) = edges.pop() {
            let mut edge_loop = Vec::<Edge>::new();
            edge_loop.push(edge);
            while edge_loop[0].start != edge_loop.last().unwrap().end {
                for next_edge in edges.iter() {
                    if edge_loop.last().unwrap().end == next_edge.start {
                        edge_loop.push(next_edge.clone());
                        edge = next_edge.clone();
                    }
                }
            }
            result.push(EdgeLoop::new(edge_loop));
        }

        result
    }

    pub fn split_parts(&self, other: &FaceGroup) -> (FaceGroup, FaceGroup, Vec<FaceGroup>) {
        let split_loops = self.intersect(other);

        let a_without_b = Vec::<Rc<Face>>::new();
        a_without_b.extend(self.subsurface(&split_loops));
        a_without_b.extend(other.subsurface(split_loops.iter().map(|edge_loop| edge_loop.flip())));
        let a_without_b = FaceGroup::new(a_without_b);

        let b_without_a = Vec::<Rc<Face>>::new();
        b_without_a.extend(other.subsurface(&split_loops));
        b_without_a.extend(self.subsurface(split_loops.iter().map(|edge_loop| edge_loop.flip())));
        let b_without_a = FaceGroup::new(b_without_a);

        let int = Vec::<FaceGroup>::new();
        for intersection in split_loops {
            let faces_s = self.subsurface(&intersection.flip());
            let faces_o = other.subsurface(&intersection.flip());
            faces_s.extend(faces_o);
            int.push(FaceGroup::new(faces_s));
        }
        return (a_without_b, b_without_a, int);
    }
}