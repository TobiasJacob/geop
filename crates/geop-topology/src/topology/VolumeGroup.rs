use std::rc::Rc;

use geop_geometry::geometry::points::point3d::Point3d;

use super::Volume::Volume;

pub struct VolumeGroup {
    pub volumes: Vec<Rc<Volume>>
}

impl VolumeGroup {
    pub fn new(volumes: Vec<Rc<Volume>>) -> VolumeGroup {
        VolumeGroup {
            volumes
        }
    }

    pub fn rasterize(&self) -> Vec<Vec<Vec<Vec<Point3d>>>> {
        todo!("Rasterize the volume group")
    }
}