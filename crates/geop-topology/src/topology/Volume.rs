use super::FaceGroup::FaceGroup;


pub struct Volume {
    pub outer_face: Rc<FaceGroup>,
    pub inner_faces: Vec<Rc<FaceGroup>>
}

impl Volume {
    pub fn new(outer_face: Rc<FaceGroup>, inner_faces: Vec<Rc<FaceGroup>>) -> Volume {
        Volume {
            outer_face,
            inner_faces
        }
    }

    pub fn rasterize(&self) -> Vec<Vec<Vec<Point3d>>> {
        todo!("Rasterize the volume")
    }
}