use std::rc::Rc;

use geop_geometry::geometry::points::point::Point;

use super::Face::Face;


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

    pub fn rasterize(&self) -> Vec<Vec<Point>> {
        self.faces.iter().flat_map(|face| face.rasterize()).collect()
    }

    fn inner_intersections(&self, other: &FaceGroup) -> Vec<Edge> {
        let mut intersections = Vec::new();
        for face in &self.faces {
            for other_face in &other.faces {
                intersections.append(&mut face.inner_intersections(other_face));
            }
        }
        intersections
    }

    fn remesh(&self, other: &EdgeLoop) -> (Vec<Face>, Vec<Face>) {

    }

    pub fn split(&self, other: &FaceGroup) -> Vec<FaceGroup> {
        todo!("Splitting...")
    }
}