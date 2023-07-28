use std::rc::Rc;

use geop_geometry::geometry::points::point::Point;

use super::{Face::Face, edge::EdgeLoop::EdgeLoop};


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

    fn get_subface(&self, border: &EdgeLoop) -> Result<FaceGroup, &'static str> {
        todo!("Subface")
    }

    fn intersections(&self, other: &FaceGroup) -> Vec<EdgeLoop> {
        todo!("inner_intersections")
    }

    fn remesh_self_other(&self, other: &FaceGroup) -> (Vec<FaceGroup>, Vec<FaceGroup>) {

    }

    pub fn split(&self, other: &FaceGroup) -> Vec<FaceGroup> {
        todo!("Splitting...")
    }
}