use std::rc::Rc;

use super::{vertex::Vertex, face::{Face, FaceIntersection}, edge::Edge, contour::Contour};

pub struct Object {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    hole_loops: Vec<Contour>,
    multiplicity: usize,
    genus: usize
}

pub enum ObjectIntersection {
    TouchingContour(Contour),
    CrossingContour(Contour),
    TouchingVertex(Vertex)
}

impl Object {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>, faces: Vec<Face>) -> Object {
        Object {
            vertices,
            edges,
            faces,
            hole_loops: todo!(),
            multiplicity: todo!(),
            genus: todo!(),
        }
    }

    pub fn validate(&self) {
        let v = self.vertices.len();
        let e = self.edges.len();
        let f = self.faces.len();
        let h = self.hole_loops.len();
        let m = self.multiplicity;
        let g = self.genus;
        assert!(v - e + f - h == 2 * (m - g));
    }

    pub fn intersect(&self, other: &Object) -> Vec<Rc<ObjectIntersection>> {
        todo!("Implement intersect");
    }

    // Remeshes this object with another object, dividing it into disjoint non-intersecting faces in 6 categories.
    pub fn remesh(&self, other: &Object) -> (Vec<Rc<Face>>, Vec<Rc<Face>>, Vec<Rc<Face>>, Vec<Rc<Face>>, Vec<Rc<Face>>, Vec<Rc<Face>>) {
        todo!("Implement remesh");
        // let intersections = Vec::<FaceIntersection>::new();
        // for face in &self.faces {
        //     for other_face in &other.faces {
        //         let intersections = face.intersect(other_face);
        //         if intersections.len() > 0 {
        //             intersections.push(intersections);
        //         }
        //     }
        // }
        // let a_out_b = Vec::<Rc<Face>>::new();
        // let a_in_b = Vec::<Rc<Face>>::new();
        // let a_on_b = Vec::<Rc<Face>>::new();
        // let b_out_a = Vec::<Rc<Face>>::new();
        // let b_in_a = Vec::<Rc<Face>>::new();
        // let b_on_a = Vec::<Rc<Face>>::new();
        // for face in self.faces {
        //     let parts = face.split(intersections);
        //     for part in parts {
        //         match other.contains(part.a_random_point()) {
        //             Contains::Yes => a_in_b.push(part),
        //             Contains::OnSurface => a_on_b.push(part),
        //             Contains::No => a_out_b.push(part),
        //         }
        //     }
        // }
        // return (a_out_b, a_in_b, a_on_b, b_out_a, b_in_a, b_on_a);
    }

    pub fn union(&self, other: &Object) {
        todo!("Implement union");
        // let (a_out_b, a_in_b, a_on_b, b_out_a, b_in_a, b_on_a) = self.remesh(other);
        // // only keep a_out_b, a_on_b, b_out_a
        // for face in a_in_b {
        //     face.remove();
        // }
        // for face in b_in_a {
        //     face.remove();
        // }
        // for face in b_on_a {
        //     face.remove();
        // }
    }
}