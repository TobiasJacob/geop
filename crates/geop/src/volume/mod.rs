use crate::{point::Point, transforms::Transform};

use super::{
    face::Face,
    shell::{Shell, ShellNormal},
};

#[derive(Clone, Debug)]
pub struct Volume {
    pub boundary: Shell,      // Normal pointing outwards
    pub cavities: Vec<Shell>, // Normal pointing inwards
}

impl Volume {
    pub fn new(boundary: Shell, cavities: Vec<Shell>) -> Volume {
        Volume { boundary, cavities }
    }

    pub fn transform(&self, transform: Transform) -> Volume {
        Volume {
            boundary: self.boundary.transform(transform),
            cavities: self
                .cavities
                .iter()
                .map(|h| h.transform(transform))
                .collect(),
        }
    }

    pub fn all_faces(&self) -> Vec<Face> {
        let mut faces = Vec::<Face>::new();

        faces.extend(self.boundary.faces.clone());
        for hole in self.cavities.iter() {
            faces.extend(hole.faces.clone());
        }
        return faces;
    }

    pub fn boundary_normal(&self, p: Point) -> ShellNormal {
        // if shell_point_contains(&self.boundary, p) != FacePointContains::Outside {
        //     return self.boundary.normal(p);
        // }
        // for hole in self.cavities.iter() {
        //     if shell_point_contains(hole, p) != FacePointContains::Outside {
        //         return hole.normal(p);
        //     }
        // }
        panic!("Point is not on boundary");
    }
}
