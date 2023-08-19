// use std::rc::Rc;

// use geop_geometry::geometry::points::point::Point;

// use super::{Face::Face, edge::{EdgeLoop::EdgeLoop}, Vertex};

use std::rc::Rc;

use geop_geometry::points::point::Point;

use super::{face::Face, edge_loop::EdgeLoop, vertex::Vertex};


// A watertight group of faces.
pub struct FaceGroup {
    pub faces: Vec<Rc<Face>>
}

enum FaceGroupIntersection {
    Face(Face),
    EdgeLoop(EdgeLoop),
    Vertex(Vertex),
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

    pub fn subsurface(&self, edge_loops: &Vec<EdgeLoop>) -> Vec<Rc<Face>> {
        todo!("Implement subsurface");
    }

    pub fn intersect(&self, other: &FaceGroup) -> Vec<Rc<FaceGroupIntersection>> {
        todo!("Implement intersect");
    }

    pub fn split_parts(&self, other: &FaceGroup) -> (FaceGroup, FaceGroup, Vec<FaceGroup>) {
        let intersections = self.intersect(other);
        let split_loops = Vec::<EdgeLoop>::new();
        for intersection in intersections {
            match *intersection {
                FaceGroupIntersection::Face(face) => {
                    split_loops.extend(face.boundaries);
                },
                FaceGroupIntersection::EdgeLoop(edge_loop) => {
                    split_loops.push(edge_loop);
                },
                FaceGroupIntersection::Vertex(vertex) => {
                    // Ignore
                },
            }
        }
        let a_without_b = Vec::<Rc<Face>>::new();
        a_without_b.extend(self.subsurface(&split_loops));
        a_without_b.extend(other.subsurface(split_loops.iter().map(|edge_loop| edge_loop.flip())));
        let a_without_b = FaceGroup::new(a_without_b);

        let b_without_a = Vec::<Rc<Face>>::new();
        b_without_a.extend(other.subsurface(&split_loops));
        b_without_a.extend(self.subsurface(split_loops.iter().map(|edge_loop| edge_loop.flip())));
        let b_without_a = FaceGroup::new(b_without_a);

        let union = Vec::<FaceGroup>::new();
        for intersection in split_loops {
            let faces_s = self.subsurface(&intersection.flip());
            let faces_o = other.subsurface(&intersection.flip());
            faces_s.extend(faces_o);
            union.push(FaceGroup::new(faces_s));
        }
        return (a_without_b, b_without_a, union);
    }
}