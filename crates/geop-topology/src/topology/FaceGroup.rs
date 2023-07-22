use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;

use super::Face::Face;


pub struct FaceGroup {
    pub faces: Vec<Rc<Face>>
}

impl FaceGroup {
    pub fn new(faces: Vec<Rc<Face>>) -> FaceGroup {
        FaceGroup {
            faces
        }
    }

    pub fn rasterize(&self) -> Vec<Vec<Point3d>> {
        self.faces.iter().flat_map(|face| face.rasterize()).collect()
    }
}