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
        self.volumes.iter().flat_map(|volume| volume.rasterize()).collect()
    }
}