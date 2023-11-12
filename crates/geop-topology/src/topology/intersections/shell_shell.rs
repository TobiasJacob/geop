use crate::topology::{volume::{Volume, VolumeShellIntersection}, intersections::edge_edge::EdgeEdgeIntersection};


pub fn shell_shell_intersect(volume_self: &Volume, volume_other: &Volume) -> Vec<VolumeShellIntersection> {
    let intersections = Vec::<EdgeEdgeIntersection>::new();
    for face in volume_self.faces.iter() {
        for other_face in volume_other.faces.iter() {
            // intersections.extend(face.intersect(&other_face));
        }
    }

    todo!("Volume::intersect")
}