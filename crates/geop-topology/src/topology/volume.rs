// use std::rc::Rc;

// use geop_geometry::geometry::points::point::Point;

// use super::{Face::Face, edge::{Contour::Contour}, Vertex};

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

    pub fn subsurface(&self, contours: &Vec<Contour>) -> Vec<Rc<Face>> {
        let mut result = Vec::<Rc<Face>>::new();
        for face in &self.faces {
            let mut face = face.clone();
            for contour in contours {
                face = face.subsurface(*contour);
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
                    todo!("Todo");
                },
                FaceIntersection::Contour(contour) => {
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

        while let Some(mut edge) = edges.pop() {
            let mut contour = Vec::<Rc<Edge>>::new();
            contour.push(Rc::new(edge.clone()));
            while contour[0].start != contour.last().unwrap().end {
                for next_edge in edges.iter() {
                    if contour.last().unwrap().end == next_edge.start {
                        contour.push(Rc::new(next_edge.clone()));
                        edge = next_edge.clone();
                    }
                }
            }
            result.push(Contour::new(contour));
        }

        result
    }

    pub fn split_parts(&self, other: &Volume) -> (Volume, Volume, Vec<Volume>) {
        todo!("Implement split_parts");
        // let split_loops = self.intersect(other);

        // let a_without_b = Vec::<Rc<Face>>::new();
        // a_without_b.extend(self.subsurface(&split_loops));
        // a_without_b.extend(other.subsurface(split_loops.iter().map(|contour| contour.neg())));
        // let a_without_b = Volume::new(a_without_b);

        // let b_without_a = Vec::<Rc<Face>>::new();
        // b_without_a.extend(other.subsurface(&split_loops));
        // b_without_a.extend(self.subsurface(split_loops.iter().map(|contour| contour.neg())));
        // let b_without_a = Volume::new(b_without_a);

        // let int = Vec::<Volume>::new();
        // for intersection in split_loops {
        //     let faces_s = self.subsurface(&intersection.neg());
        //     let faces_o = other.subsurface(&intersection.neg());
        //     faces_s.extend(faces_o);
        //     int.push(Volume::new(faces_s));
        // }
        // return (a_without_b, b_without_a, int);
    }
}