// use std::rc::Rc;

// use geop_geometry::geometry::points::point::Point;

// use super::{Face::Face, edge::{EdgeLoop::EdgeLoop}, Vertex};

use std::rc::Rc;

use geop_geometry::points::point::Point;

use super::{face::{Face, FaceIntersection}, contour::Contour, vertex::Vertex, edge::Edge};


// A watertight group of faces.
pub struct Volume {
    pub faces: Vec<Rc<Face>>
}

impl Volume {
    pub fn new(faces: Vec<Rc<Face>>) -> Volume {
        Volume {
            faces
        }
    }

    pub fn contains(&self, other: &Point) -> bool {
        todo!("Implement contains");
    }

    pub fn subsurface(&self, contours: &Vec<Contour>) -> Vec<Vec<Rc<Face>>> {
        let result = Vec::<Rc<Face>>::new();
        for face in &self.faces {
            let mut face = face.clone();
            for contour in contours {
                face = face.split_if_necessary(contour);
            }
            result.push(face);
        }
        result
    }

    pub fn remesh(&self, other: Volume) {

    }

    pub fn intersect(&self, other: &Volume) -> Vec<Contour> {
        let mut face_intersections: Vec<FaceIntersection> = Vec::new();

        for face in &self.faces {
            for other_face in &other.faces {
                let intersections = face.intersect(other_face);
                if intersections.len() > 0 {
                    face_intersections.extend(intersections);
                }
            }
        }

        let result: Vec<Contour> = Vec::new();
        let edges: Vec<Edge> = Vec::new();

        for intersect in face_intersections {
            match intersect {
                FaceIntersection::Face(face) => {
                    result.extend(face.boundaries);
                },
                FaceIntersection::EdgeLoop(contour) => {
                    result.push(contour);
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

        let mut contour = Vec::<Edge>::new();
        while let Some(edge) = edges.pop() {
            let mut contour = Vec::<Edge>::new();
            contour.push(edge);
            while contour[0].start != contour.last().unwrap().end {
                for next_edge in edges.iter() {
                    if contour.last().unwrap().end == next_edge.start {
                        contour.push(next_edge.clone());
                        edge = next_edge.clone();
                    }
                }
            }
            result.push(Contour::new(contour));
        }

        result
    }

    pub fn split_parts(&self, other: &Volume) -> (Volume, Volume, Vec<Volume>) {
        let split_loops = self.intersect(other);

        let a_without_b = Vec::<Rc<Face>>::new();
        a_without_b.extend(self.subsurface(&split_loops));
        a_without_b.extend(other.subsurface(split_loops.iter().map(|contour| contour.flip())));
        let a_without_b = Volume::new(a_without_b);

        let b_without_a = Vec::<Rc<Face>>::new();
        b_without_a.extend(other.subsurface(&split_loops));
        b_without_a.extend(self.subsurface(split_loops.iter().map(|contour| contour.flip())));
        let b_without_a = Volume::new(b_without_a);

        let int = Vec::<Volume>::new();
        for intersection in split_loops {
            let faces_s = self.subsurface(&intersection.flip());
            let faces_o = other.subsurface(&intersection.flip());
            faces_s.extend(faces_o);
            int.push(Volume::new(faces_s));
        }
        return (a_without_b, b_without_a, int);
    }
}